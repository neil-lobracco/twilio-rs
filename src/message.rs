use crate::{Client, FromMap, Post, TwilioError};
use serde_derive::Deserialize;

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
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MessageStatus {
    queued,
    sending,
    sent,
    failed,
    delivered,
    undelivered,
    receiving,
    received,
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
    pub fn send_message(&self, msg: OutboundMessage) -> Result<Message, TwilioError> {
        let opts = [("To", &*msg.to), ("From", &*msg.from), ("Body", &*msg.body)];
        self.send_request(Post, "Messages", &opts)
    }
}

impl FromMap for Message {
    fn from_map(m: &::std::collections::HashMap<&str, &str>) -> Result<Box<Message>, TwilioError> {
        let from = match m.get("From") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let to = match m.get("To") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let sid = match m.get("MessageSid") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let body = m.get("Body");
        Ok(Box::new(Message {
            from: from.to_string(),
            to: to.to_string(),
            sid: sid.to_string(),
            body: body.map(|s| s.to_string()),
            status: None,
        }))
    }
}
