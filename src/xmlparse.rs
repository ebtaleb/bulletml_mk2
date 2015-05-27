#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xml;
extern crate tree;

use std::fs::File;
use std::collections::HashMap;

use xml::reader::EventReader;
use xml::reader::events::*;

use tree::bmltree::*;

fn indent(size: usize) -> String {
    let indent: &'static str = "    ";
    (0..size).map(|_| indent)
             .fold(String::with_capacity(size*indent.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
    info!("starting up");

    let file = File::open("src/sample_2.xml").unwrap();
    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut curr_tag = "".to_string();
    let mut parent_tag = "".to_string();

    let mut parent_label = "".to_string();
    let mut curr_label = "".to_string();

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {

                curr_tag = name.local_name.to_string();

                if parent_tag == "" {
                    parent_tag = name.local_name.to_string()
                } else {
                    parent_tag = curr_tag
                }

                println!("{}+{} {:?}", indent(depth), name.local_name, attributes);
                depth += 1;

                match name.local_name.as_ref() {
                    "bulletml"
                    | "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" => {
                        match attributes.is_empty() {
                            true => {}
                            false =>  { curr_label = attributes[0].value.to_string();
                                        let label_name = attributes[0].value.to_string();
                            }
                        }
                    }

                    _ => {}
                }
            }

            XmlEvent::EndElement { name } => {
                //if curr_tag == name.local_name {
                    //println!("building tag {}", name)
                //}
                depth -= 1;
                println!("{}-{}", indent(depth), &name.local_name);
                curr_tag = parent_tag.to_string();
            }

            XmlEvent::Characters(s) => {
                println!("{} {}", indent(depth), s);
            }

            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break;
            }

            _ => {}
        }
    }
}
