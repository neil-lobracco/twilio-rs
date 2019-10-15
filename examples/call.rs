use std::env;
extern crate twilio;
use twilio::{Client,Call,OutboundCall};
fn main() {
    let to = &env::var("TWILIO_ACCOUNT_PHONE_NUMBER").unwrap();
    let from = &env::var("TWILIO_TESTER_PHONE_NUMBER").unwrap();
    let url = "http://demo.twilio.com/welcome/voice/";
    let app_id = &env::var("TWILIO_APP_ID").unwrap();
    let auth_token = &env::var("TWILIO_AUTH_TOKEN").unwrap();
    let client = Client::new(app_id,auth_token);
    let call = OutboundCall::new(from,to,url);
    match client.make_call(call) {
        Err(e) => println!("{:?}",e),
        Ok(m)  => println!("{:?}",m),
    }
}
