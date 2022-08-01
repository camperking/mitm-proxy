use std::fs::File;
use std::io::prelude::*;

use rcgen::{self, DistinguishedName, CertificateParams, Certificate};


fn main() {

    let hostnames = vec!["localhost".to_string(), "demo.demo".to_string()];

    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::OrganizationName, "Demo");
    dn.push(rcgen::DnType::CountryName, "DE");
    dn.push(rcgen::DnType::CommonName, "demo.demo");
    
    let mut cert_params = CertificateParams::new(hostnames);
    cert_params.distinguished_name = dn;
    cert_params.serial_number = Option::Some(1);
    cert_params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    cert_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

    let certificate = Certificate::from_params(cert_params).unwrap();

    let cert = certificate.serialize_pem().unwrap();
    let key = certificate.serialize_private_key_pem();


    let mut cert_file = File::create("cert.pem").unwrap();
    cert_file.write_all(cert.as_bytes()).unwrap();
    cert_file.flush().unwrap();
    
    let mut key_file = File::create("key.pem").unwrap();
    key_file.write_all(key.as_bytes()).unwrap();
    key_file.flush().unwrap();

    
}