use twiml::{Action,format_xml_string};
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
