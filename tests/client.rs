use std::env;
use twilio::{OutboundMessage, TwilioClient};

#[tokio::test]
async fn send_sms() {
    // dotenv::dotenv().ok();

    let account_id = env::var("TWILIO_ACCOUNT_SID").expect("Find ACCOUNT_ID environment variable");
    let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("Find AUTH_TOKEN environment variable");
    let from = env::var("MY_TWILIO_NUMBER").expect("Find FROM environment variable");
    let to = env::var("MY_PHONE_NUMBER").expect("Find TO environment variable");

    let client = TwilioClient::new(&account_id, &auth_token);
    client
        .send_message(OutboundMessage::new(&from, &to, "Hello, World!"))
        .await
        .expect("to send message");
}
