use crate::{Client, FromMap, TwilioError, POST};
use serde::Deserialize;
use std::collections::BTreeMap;

pub struct OutboundMessage<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub body: &'a str,
}

impl<'a> OutboundMessage<'a> {
    pub fn new(from: &'a str, to: &'a str, body: &'a str) -> OutboundMessage<'a> {
        OutboundMessage { from, to, body }
    }
}

/// See the Twilio docs on [Message Status values](https://www.twilio.com/docs/sms/api/message-resource#message-status-values)
/// for descriptions of the message status values.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    Queued,
    Sending,
    Sent,
    Failed,
    Delivered,
    Undelivered,
    Receiving,
    Received,
    Accepted,
    Scheduled,
    Read,
    PartiallyDelivered,
    Canceled,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub body: Option<String>,
    pub sid: String,
    pub status: Option<MessageStatus>,
}

impl Client {
    pub async fn send_message(&self, msg: OutboundMessage<'_>) -> Result<Message, TwilioError> {
        let opts = [("To", &*msg.to), ("From", &*msg.from), ("Body", &*msg.body)];
        self.send_request(POST, "Messages", &opts).await
    }
}

impl FromMap for Message {
    fn from_map(mut m: BTreeMap<String, String>) -> Result<Box<Message>, TwilioError> {
        let from = match m.remove("From") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let to = match m.remove("To") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let sid = match m.remove("MessageSid") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let body = m.remove("Body");
        Ok(Box::new(Message {
            from,
            to,
            sid,
            body,
            status: None,
        }))
    }
}
