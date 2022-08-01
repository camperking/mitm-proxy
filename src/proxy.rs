use std::convert::Infallible;
use std::fs::File;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::client::conn::Builder;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{Body, Method, Request, Response, http};
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};
use std::io::Read;
use tokio::net::{TcpListener, TcpStream};
use rcgen::{Certificate, CertificateParams, DnType, KeyPair};

use crate::events::{Events};
use crate::events::EventsBuilder;

pub struct Proxy {
    events: Events
}

impl Proxy {
    pub fn new(events: EventsBuilder) -> (Proxy, Events) {
        (
            Proxy {
                events: events.0,
            },
            Events {
                request: events.1.request,
                response: events.1.response
            }
        )
    }

    pub async fn listen(&mut self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            let events = self.events.clone();

            tokio::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(move |req| {
                        proxy_service(req, events.clone())
                    }))
                    .with_upgrades()
                    .await
                {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }

}

async fn proxy_service(
    req: Request<Body>,
    events: Events
) -> Result<Response<Body>, hyper::Error> {

    events.request.0.send(format!("{:?}", req)).unwrap();

    if Method::CONNECT == req.method() {
        // Received an HTTP request like:
        // ```
        // CONNECT www.domain.com:443 HTTP/1.1
        // Host: www.domain.com:443
        // Proxy-Connection: Keep-Alive
        // ```
        //
        // When HTTP method is CONNECT we should return an empty body
        // then we can eventually upgrade the connection and talk a new protocol.
        //
        // Note: only after client received an empty body with STATUS_OK can the
        // connection be upgraded, so we can't return a response inside
        // `on_upgrade` future.
        let uri = req.uri().clone();

        if let Some(addr) = host_addr(req.uri()) {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, uri).await {
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(Body::empty()))
        } else {
            eprintln!("CONNECT host is not socket addr: {:?}", req.uri());
            let mut resp = Response::new(Body::from("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        let host = req.uri().host().expect("uri has no host");
        let port = req.uri().port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await.unwrap();

        let (mut sender, conn) = Builder::new()
            .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .handshake(stream)
            .await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        sender.send_request(req).await
    }
}


fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().and_then(|auth| Some(auth.to_string()))
}


    // we need a CA cert and installed in the browser
    // for each tls connection we sign a new certificate as the CA
async fn tunnel(upgraded: Upgraded, uri: http::Uri) -> std::io::Result<()> {
    let hostname = uri.host().unwrap().to_string();

    let (cert, key) = generate_cert(&hostname);

    let server_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![rustls::Certificate(cert)], rustls::PrivateKey(key))
        .unwrap();
    let server_config = Arc::new(server_config);

    let tls_server = TlsAcceptor::from(server_config);
    

    let stream = tls_server.accept(upgraded).await.unwrap();

    Http::new()
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve_connection(stream, service_fn(mitm))
        .await
        .unwrap();
                
    Ok(())
  
}

fn generate_cert(hostname: &String) -> (Vec<u8>, Vec<u8>) {

    let hostnames = vec![hostname.clone(), "localhost".to_string()];

    let ca = load_ca();
    
    let mut params = CertificateParams::new(hostnames); 
 	params.distinguished_name.push(DnType::OrganizationName, "Crab widgits SE"); 
 	params.distinguished_name.push(DnType::CommonName, "Dev domain"); 
  
 	let cert = Certificate::from_params(params).unwrap();
    
    let key = cert.serialize_private_key_pem();
    let key = pem::parse(key).unwrap().contents;

    let cert_der = cert.serialize_der_with_signer(&ca).unwrap(); 

    (cert_der, key)
}

fn load_ca() -> Certificate {
    
    let mut cert_file = File::open("cert.pem").unwrap();
    let mut cert = String::new();
    cert_file.read_to_string(&mut cert).unwrap();

    let mut key_file = File::open("key.pem").unwrap();
    let mut key = String::new();
    key_file.read_to_string(&mut key).unwrap();

    let key_pair = KeyPair::from_pem(&key).unwrap();

    let params = CertificateParams::from_ca_cert_pem(&cert, key_pair).unwrap();

    Certificate::from_params(params).unwrap()

}

async fn mitm(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:#?}", req.headers());
    Ok(Response::new(Body::from("Hello World!")))
 }