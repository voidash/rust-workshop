//! *Thirty-Rust* is for 30 days of Rust 

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    email: &'static str,
    #[default("")]
    password: &'static str,

    // firebase relatied
    #[default("")]
    something: &'static str,
}

/// requires party to send to and email body in html
fn send_mail(to: String, email_body:String) {
    let config = CONFIG;
    let email = Message::builder()
        .from("Ashish <ashish.thapa477@gmail.com>".parse().unwrap())
        .to(format!("coke <{}>",to).parse().unwrap())
        .subject("From Rust")
        .header(ContentType::TEXT_HTML)
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(config.email.to_owned(), config.password.to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

fn main() {
    send_mail("cokevaidhya@gmail.com".to_string(),r#"
               <h1>Tum Kyu chale aate ho</h1>
               "#.to_string());

}
