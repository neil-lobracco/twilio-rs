use twilio::{Client, OutboundCall};

#[tokio::main]
async fn main() {
    let to = "<to-number>";
    let from = "<from-number>";
    let url = "https://demo.twilio.com/welcome/voice/";
    let app_id = "my_app_id";
    let auth_token = "my_auth_token";
    let client = Client::new(app_id, auth_token);
    let call = OutboundCall::new(from, to, url);
    match client.make_call(call).await {
        Ok(m) => println!("{:?}", m),
        Err(e) => eprintln!("{:?}", e),
    }
}
