pub trait Action {
    fn as_twiml(&self) -> String;
}

pub struct Twiml {
    body : String,
}

impl Twiml {
    pub fn new() -> Twiml {
        Twiml { body : "".to_string() }
    }
    pub fn add(&mut self, a: &Action) -> &Self {
        let twiml = a.as_twiml();
        self.body.push_str((&twiml as &AsRef<str>).as_ref());
        self
    }
    pub fn as_twiml(&self) -> String {
        let b: &str = self.body.as_ref();
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?><Response>{}</Response>",b)
    }
}

fn format_xml_string(tag: &str, attributes: &[(&str,&str)], inner: &str) -> String {
    let attribute_string = match attributes.len(){
        0 => "".to_string(),
        _ => attributes.iter().map(|t|{
            format!("{}=\"{}\"",t.0,t.1)
        }).fold("".to_string(),|mut acc,v|{
            acc.push_str(" ");
            acc.push_str(&v);
            acc
        })
    };
    let attribute_str: &str = attribute_string.as_ref();
    format!("<{}{}>{}</{}>",tag,attribute_str,inner,tag)
}

pub struct Message {
    txt : String,
}
impl Action for Message {
    fn as_twiml(&self) -> String {
        format_xml_string("Message",&vec![],&self.txt)
    }
}
impl Message {
    pub fn new(txt: String) -> Message {
        Message { txt: txt }
    }
}

pub enum Method {
    Get,
    Post,
}

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
