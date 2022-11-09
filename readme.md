## mitm proxy in pure rust

The code is a complete mess, so right now its just a proof of concept. This code is actually three months old. I would do things differently now.
Tested with Firefox

first create a new certificate authority

` cargo run --example gen_ca_cert`

import ` cert.pem ` as a new certificate authority in your browser

set browser proxy to localhost:9000 and allow https via proxy

then run the example

` cargo run --example events `
