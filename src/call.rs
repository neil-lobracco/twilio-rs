pub struct OutboundCall<'a> {
    pub from : &'a str,
    pub to   : &'a str,
    pub url : &'a str,
}
impl<'a> OutboundCall<'a> {
    pub fn new(from: &'a str,to: &'a str, url: &'a str) -> OutboundCall<'a> {
        OutboundCall { from: from, to: to, url: url }
    }
}
#[derive(RustcDecodable)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CallStatus {
    queued,
    ringing,
    inprogress,
    canceled,
    completed,
    failed,
    busy,
    noanswer,
}
#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Call {
    from : String,
    to   : String,
    sid  : String,
    status : CallStatus,
}
impl ::Client {
    pub fn make_call(&self, call: OutboundCall) -> Result<Call,::TwilioError> {
        let opts = [("To",&*call.to),("From",&*call.from),("Url",&*call.url)];
        self.send_request(::Post,"Calls",&opts)
    }
}
