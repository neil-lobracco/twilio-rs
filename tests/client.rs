use std::env;
use twilio::{Client, OutboundMessage};

#[tokio::test]
async fn send_sms() {
    dotenv::dotenv().ok();

    let account_id = env::var("ACCOUNT_ID").expect("Find ACCOUNT_ID environment variable");
    let auth_token = env::var("AUTH_TOKEN").expect("Find AUTH_TOKEN environment variable");
    let from = env::var("FROM").expect("Find FROM environment variable");
    let to = env::var("TO").expect("Find TO environment variable");

    let client = Client::new(&account_id, &auth_token);
    client
        .send_message(OutboundMessage::new(&from, &to, "Hello, World!"))
        .await
        .expect("to send message");
}
