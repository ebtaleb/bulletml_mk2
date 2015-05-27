#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xml;
extern crate tree;

use std::fs::File;
use std::collections::HashMap;

use xml::reader::EventReader;
use xml::reader::events::*;

use tree::bmltree::{TreeNode, Name, string2name, string2type};

fn indent(size: usize) -> String {
    let indent: &'static str = "    ";
    (0..size).map(|_| indent)
             .fold(String::with_capacity(size*indent.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    let file = File::open("src/sam.xml").unwrap();
    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut parent_tag = "".to_string();
    let mut curr_tag = "".to_string();

    let mut parent_label = "".to_string();
    let mut curr_label = "".to_string();

    let mut parent_node = TreeNode::new_box("none");
    let mut curr_node = parent_node.clone();

    let mut tag_stack : Vec<Name> = Vec::new();

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {

                curr_tag = name.local_name.to_string();

                if parent_tag == "" {
                    parent_tag = name.local_name.to_string()
                } else {
                    parent_tag = curr_tag.to_string()
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

                        tag_stack.push(string2name(&name.local_name));

                        info!("pushing {}", &name.local_name);
                        parent_node = curr_node;
                        curr_node = TreeNode::new_box(&name.local_name);

                        match attributes.is_empty() {
                            true => {}
                            false =>  {
                                match attributes[0].name.local_name.as_ref() {
                                    "label" =>  {
                                        curr_label = attributes[0].value.to_string();

                                        if parent_label == "" {
                                            parent_label = attributes[0].value.to_string();
                                        } else {
                                            parent_label = curr_label
                                        }


                                    },

                                    "type" => { (*curr_node).set_type(&attributes[0].value) },
                                    _ => {}
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }

            XmlEvent::EndElement { name } => {

                info!("popping {:?}", tag_stack.pop());

                match name.local_name.as_ref() {
                    "bulletml"
                    | "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" =>
                        {
                            if curr_tag == name.local_name {

                                (*parent_node).add_child(curr_node);

                                depth -= 1;
                                println!("{}-{}", indent(depth), &name.local_name);

                                curr_tag = parent_tag.to_string();
                                curr_label = parent_label.to_string();
                                curr_node = parent_node.clone();

                            }
                        }

                    _ => {}
                }

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

    println!("{:?}", tag_stack);
}
