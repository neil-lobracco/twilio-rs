use super::{format_xml_string, Action, Method};

pub struct Redirect {
    pub url: String,
    pub method: Method,
}

impl Action for Redirect {
    fn as_twiml(&self) -> String {
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        format_xml_string("Redirect", &vec![("method", method_str)], &self.url)
    }
}
