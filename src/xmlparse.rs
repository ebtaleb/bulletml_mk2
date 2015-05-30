#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xml;
extern crate bml;

extern crate typed_arena;
extern crate arena_tree;

use std::env;
use std::fs::File;

use xml::reader::EventReader;
use xml::reader::events::*;
use bml::bml::{BulletMLNode, Name, string2name};

fn indent(size: usize) -> String {
    let indent: &'static str = "    ";
    (0..size).map(|_| indent)
             .fold(String::with_capacity(size*indent.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut parent_tag = "".to_string();
    let mut curr_tag = "".to_string();

    let mut parent_label = "".to_string();
    let mut curr_label = "".to_string();

    let arena = typed_arena::Arena::new();
    //let root = arena.alloc(arena_tree::Node::new(BulletMLNode::new_cell("bulletml")));
    let mut curr_node = arena.alloc(arena_tree::Node::new(BulletMLNode::new_cell("none")));

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
                    "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" => {

                        tag_stack.push(string2name(&name.local_name));

                        info!("pushing {}", &name.local_name);
                        curr_node = arena.alloc(arena_tree::Node::new(BulletMLNode::new_cell(&name.local_name)));

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

                                    "type" => { curr_node.data.borrow_mut().set_type(&attributes[0].value) },
                                    _ => {}
                                }
                            }
                        }
                    }

                    // bulletml ignored, we start with a bulletml node directly
                    _ => {}
                }
            }

            XmlEvent::EndElement { name } => {

                info!("popping {:?}", tag_stack.pop());

                match name.local_name.as_ref() {
                    "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" =>
                        {
                            if curr_tag == name.local_name {

                                depth -= 1;
                                println!("{}-{}", indent(depth), &name.local_name);

                                curr_tag = parent_tag.to_string();
                                curr_label = parent_label.to_string();
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
