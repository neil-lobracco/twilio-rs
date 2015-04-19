use twiml::{Action,format_xml_string,Method};
pub struct Redirect {
    url : String,
    method : Method,
}
impl Action for Redirect {
    fn as_twiml(&self) -> String {
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        format_xml_string("Redirect",&vec![("method",method_str)],&self.url)
    }
}
impl Redirect {
    pub fn new(url: String, method: Method) -> Redirect {
        Redirect { url: url, method: method }
    }
}
