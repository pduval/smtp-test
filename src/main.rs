#[macro_use] extern crate quicli;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate env_logger;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;

use lettre_email::EmailBuilder;
use std::path::Path;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "to", short = "t", default_value = "philippe@metrisolve.com")]
    destination: String,
    #[structopt(long = "from", short = "f", default_value = "ulyssessupport@asai.ie")]
    from: String,
    #[structopt(long = "user", short = "u", default_value = "ulyssessupport@asai.ie")]
    username: String,
    #[structopt(long = "password", short = "P", default_value = "password")]
    password: String,
    #[structopt(long = "host", short = "h", default_value = "smtp.office365.com")]
    host: String,
    // #[structopt(long = "port", short = "p", default_value = "587")]
    // port: usize,
    // #[structopt(long = "tls", short = "s", default_value = "true")]
    // use_tls: bool,
    // #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    // verbosity: u8
}

fn main() {
    let args = Cli::from_args();
    env_logger::init();
//    error!("Got verbosity for {}: {}", env!("CARGO_PKG_NAME"), args.verbosity);
    debug!("Building Email");
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to((args.destination, "Philippe Duval"))
        // ... or by an address only
        .from(args.from)
        .subject("Hi, Hello world")
        .text("Hello world.")
        // .attachment(Path::new("Cargo.toml"), None, &mime::TEXT_PLAIN).unwrap()
        .build()
        .unwrap();
    debug!("Building mailer");
    // Open a local connection on port 587
    let mut mailer = SmtpTransport::simple_builder(args.host)
        .unwrap()
        .credentials(Credentials::new(args.username, args.password))
        .authentication_mechanism(Mechanism::Login)
        .smtp_utf8(true)
        .build();

    // Send the email
    let result = mailer.send(&email);

    if result.is_ok() {
        println!("Email sent: {:?}", result);
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}//);
