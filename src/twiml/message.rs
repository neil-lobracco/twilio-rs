use super::{Action,format_xml_string};
pub struct Message {
    pub txt : String,
}
impl Action for Message {
    fn as_twiml(&self) -> String {
        format_xml_string("Message",&vec![],&self.txt)
    }
}
