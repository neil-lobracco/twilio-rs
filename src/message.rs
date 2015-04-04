pub struct OutboundMessage<'a> {
    pub from : &'a str,
    pub to   : &'a str,
    pub body : &'a str,
}
impl<'a> OutboundMessage<'a> {
    pub fn new(from: &'a str,to: &'a str, body: &'a str) -> OutboundMessage<'a> {
        OutboundMessage { from: from, to: to, body: body }
    }
}
#[derive(RustcDecodable)]
#[derive(Debug)]
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
#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Message {
    from : String,
    to   : String,
    body : Option<String>,
    sid  : String,
    status : MessageStatus,
}
impl ::Client {
    pub fn send_message(&self, msg: OutboundMessage) -> Result<Message,::TwilioError> {
        let opts = [("To",&*msg.to),("From",&*msg.from),("Body",&*msg.body)];
        self.send_request(::Post,"Messages",&opts)
    }
}
