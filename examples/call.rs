extern crate twilio;
use twilio::{Client,Call,OutboundCall};
fn main() {
    let to = "<to-number>";
    let from = "<from-number>";
    let url = "http://demo.twilio.com/welcome/voice/";
    let app_id = "my_app_id";
    let auth_token = "my_auth_token";
    let client = Client::new(app_id,auth_token);
    let call = OutboundCall::new(from,to,url);
    match client.make_call(call) {
        Err(e) => println!("{:?}",e),
        Ok(m)  => println!("{:?}",m),
    }
}
