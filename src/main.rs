extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate html5ever;
extern crate string_cache;
extern crate tendril;

mod readurl;
//mod printdom;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use html5ever::parse_document;
use html5ever::rcdom::{NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;
use tendril::*;

//use std::fmt::Display::fmt;
//use std::io::Read;

use std::iter::repeat;
use std::default::Default;
use std::string::String;
use std::borrow::Borrow;


fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    
    let url = readurl::readurl();

    let mut res = client.get(url).send().unwrap();
    //let mut body = vec![];
    //res.read_to_end(&mut body).unwrap();

    let dom = parse_document(RcDom::default(), Default::default())
                .from_utf8().read_from(&mut res).unwrap();
    
    walk(0, dom.document);

    //let css_vec = get_css_links(dom.document);
    // for c in css_vec.iter() {
    //     println!("{}", c);
    // }

    //println!("{}", String::from_utf8_lossy(&body));
    //println!("{:?}", dom);
}



pub fn walk(indent: usize, handle: Handle) {
        let node = handle;
        
        print!("{}", repeat(" ").take(indent).collect::<String>());
        match node.data {
            //NodeData::Document
                //=> println!("#Document"),

            //NodeData::Doctype { ref name, ref public_id, ref system_id }
                //=> println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

            //NodeData::Text { ref contents }
                //=> println!("#text: {}", escape_default(&contents.borrow())),

            //NodeData::Comment { ref contents }
                //=> println!("<!-- {} -->", escape_default(contents)),

            NodeData::Element { ref name, ref attrs, .. } => {
                assert!(name.ns == ns!(html));
                print!("<{}", name.local);
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    print!(" {}=\"{}\"", attr.name.local, attr.value);
                }
                println!(">");
            }

            _ => {
                 //don't do anything
            }
        }

        for child in node.children.borrow().iter() {
            walk(indent+1, child.clone());
        }
    }


pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}


// fn get_css_links(handle: Handle) -> Vec<String> {
//     let mut csslinks: Vec<String> = Vec::new();
//     let mut queue: Vec<Handle> = Vec::new();
//     queue.push(handle);
//     while queue.len() != 0 {
//         let handle = queue.remove(0);
//         let node = handle;
//         match node.data {
//             NodeData::Element{ref name, ref attrs, ..} => {
//                 assert!(name.ns == ns!(html));
//                 let mut is_css = false;
//                 for attr in attrs.borrow().iter() {
//                     assert!(attr.name.ns == ns!());
//                     let link = string_cache::Atom::from("link");
//                     let type = string_cache::Atom::from("type");
//                     let css = Tendril::from("text/css");
//                     if name.local == link && 
//                         attr.name.local == type && 
//                         attr.value == css {
//                             is_css = true;
//                     }
//                     let href = string_cache::Atom::from("href");
//                     if is_css && attr.name.local == href {
//                         let link = String::from(attr.value.clone());
//                         csslinks.push(link);
//                     }
//                 }
//             }
//             _ => {
//                 //don't do anything
//             }
//         }
//         for child in node.children.borrow().iter() {
//             queue.push(child.clone());
//         }
//     }
//     return csslinks;
// }