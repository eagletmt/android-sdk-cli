use std::collections::HashMap;
use std::io::Read;
use xml::reader::{EventReader, Result, XmlEvent};

#[derive(Debug)]
pub enum Event {
    StartElement { local_name: String, attributes: HashMap<String, String> },
    EndElement { local_name: String },
    Text { text: String },
}

pub fn parse<R: Read>(source: R) -> Result<Vec<Event>>{
    let reader = EventReader::new(source);
    let mut stream = Vec::new();
    for e in reader {
        match try!(e) {
            XmlEvent::StartElement { name, attributes, ..} => {
                let mut attrs = HashMap::new();
                for attr in attributes {
                    attrs.insert(attr.name.local_name, attr.value);
                }
                stream.push(Event::StartElement { local_name: name.local_name, attributes: attrs });
            }
            XmlEvent::EndElement { name, .. } => {
                stream.push(Event::EndElement { local_name: name.local_name });
            }
            XmlEvent::StartDocument { .. } => {}
            XmlEvent::EndDocument => {}
            XmlEvent::ProcessingInstruction { .. }  => {}
            XmlEvent::Comment(_) => {}
            XmlEvent::CData(text) => {
                stream.push(Event::Text { text: text });
            }
            XmlEvent::Characters(text) => {
                stream.push(Event::Text { text: text });
            }
            XmlEvent::Whitespace(_) => {}
        }
    }
    return Ok(stream);
}
