#[macro_use] extern crate html5ever;
extern crate tendril;


use std::iter::repeat;
use std::default::Default;
use std::string::String;

use self::tendril::TendrilSink;
use self::html5ever::parse_document;
use self::html5ever::rcdom::{NodeData, RcDom, Handle};


pub fn walk(indent: usize, handle: Handle) {
        let node = handle;
        // FIXME: don't allocate
        print!("{}", repeat(" ").take(indent).collect::<String>());
        match node.data {
            NodeData::Document
                => println!("#Document"),

            NodeData::Doctype { ref name, ref public_id, ref system_id }
                => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

            NodeData::Text { ref contents }
                => println!("#text: {}", escape_default(&contents.borrow())),

            NodeData::Comment { ref contents }
                => println!("<!-- {} -->", escape_default(contents)),

            NodeData::Element { ref name, ref attrs, .. } => {
                assert!(name.ns == ns!(html));
                print!("<{}", name.local);
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    print!(" {}=\"{}\"", attr.name.local, attr.value);
                }
                println!(">");
            }

            NodeData::ProcessingInstruction { .. } => unreachable!()
        }

        for child in node.children.borrow().iter() {
            walk(indent+4, child.clone());
        }
    }


pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}