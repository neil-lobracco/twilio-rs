use crate::{Client, FromMap, TwilioError, POST, GET};
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
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub date_sent: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

impl Client {
    pub async fn send_message(&self, msg: OutboundMessage<'_>) -> Result<Message, TwilioError> {
        let opts = [("To", &*msg.to), ("From", &*msg.from), ("Body", &*msg.body)];
        self.send_request(POST, "Messages", &opts).await
    }

    pub async fn get_message(&self, message_sid: String) -> Result<Message, TwilioError> {
        let opts: &[(&str, &str)] = &[];
        self.send_request(GET, format!("Messages/{}", message_sid).as_str(), &opts).await
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
            error_code: None,
            error_message: None,
            date_created: None,
            date_updated: None,
            date_sent: None
        }))
    }
}
