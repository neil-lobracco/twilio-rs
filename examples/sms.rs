extern crate twilio;
use twilio::{Client, OutboundMessage};
fn main() {
    let to = "<to-number>";
    let from = "<from-number>";
    let body = "Hello, World! ";
    let app_id = "<app-id>";
    let auth_token = "<auth-token>";
    let client = Client::new(app_id, auth_token);
    let msg = OutboundMessage::new(from, to, body);
    match client.send_message(msg) {
        Err(e) => println!("{:?}", e),
        Ok(m) => println!("{:?}", m),
    }
}
