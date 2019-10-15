use std::env;
extern crate twilio;
use twilio::{Client,OutboundMessage};
fn main() {
    let to = &env::var("TWILIO_TESTER_PHONE_NUMBER").unwrap();
    let from = &env::var("TWILIO_ACCOUNT_PHONE_NUMBER").unwrap();
    let body = "Hello, World! ";
    let app_id = &env::var("TWILIO_APP_ID").unwrap();
    let auth_token = &env::var("TWILIO_AUTH_TOKEN").unwrap();
    let client = Client::new(app_id,auth_token);
    let msg = OutboundMessage::new(from,to,body);
    match client.send_message(msg) {
        Err(e) => println!("{:?}",e),
        Ok(m)  => println!("{:?}",m),
    }
}
