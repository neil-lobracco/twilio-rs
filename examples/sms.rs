use twilio::{Client, OutboundMessage};

#[tokio::main]
async fn main() {
    let to = "<to-number>";
    let from = "<from-number>";
    let body = "Hello, World! ";
    let app_id = "<app-id>";
    let auth_token = "<auth-token>";
    let client = Client::new(app_id, auth_token);
    let msg = OutboundMessage::new(from, to, body);
    match client.send_message(msg).await {
        Ok(m) => println!("{:?}", m),
        Err(e) => eprintln!("{:?}", e),
    }
}
