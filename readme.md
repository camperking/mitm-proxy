## mitm proxy in pure rust

The code is a complete mess with hundreds of unwraps but right now its just a proof of concept. First you would have to create a new certificate authority and import it into your browser. I have tested it with Firefox.

first create a new certificate authority

` cargo run --example gen_ca_cert`

then run the example

` cargo run --example events `


