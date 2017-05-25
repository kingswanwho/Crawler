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
//use tendril::*;

//use std::fmt::Display::fmt;
//use std::io::Read;

use std::iter::repeat;
use std::default::Default;
use std::string::String;
use std::borrow::Borrow;

use std::sync::{Arc, Mutex};
use std::thread;
use std::marker::Send;

struct myrcdom{
    dom: RcDom,
}

unsafe impl Send for myrcdom {}


fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    
    let client = Client::with_connector(connector);
    
    let url = readurl::readurl();

    let mut res = client.get(url).send().unwrap();
    

    let dom = parse_document(RcDom::default(), Default::default())
                .from_utf8().read_from(&mut res).unwrap();
    
    let mut link: Vec<String> = Vec::new();
    let mut links: Vec<String> = Vec::new();
    
    let st_dom = myrcdom {dom: dom};

    let multi_dom = Arc::new(Mutex::new(st_dom.dom));

    for _ in 0..3 {
        let dom = multi_dom.clone();

        thread::spawn(move || {
            let mut dom = dom.lock().unwrap();
            let links = walk(&mut link, dom.document);

        });
    }




    

    println!("{}", links.len());

    for l in links {
        println!("{}", l);
    }

    
}



pub fn walk(mut link: &mut Vec<String>, handle: Handle)-> Vec<String> {
        let node = handle;
        //let node = Arc::new(Mutex::new(handle));

        //let mut link: Vec<String> =Vec::new();
        //print!("{}", repeat("").take(indent).collect::<String>());
        //let mut link_buf: Vec<String> = Vec::new();

        match node.data {     
            NodeData::Element { ref name, ref attrs, .. } => {
                assert!(name.ns == ns!(html));
                if name.local == string_cache::Atom::from("a") {
                    for attr in attrs.borrow().iter() {
                        assert!(attr.name.ns == ns!());
                        if attr.name.local == string_cache::Atom::from("href") {
                            //let link_str = String::from(attr.value.clone());
                            //println!("{}", attr.value);
                            link.push(String::from(attr.value.clone()));
                        }
                        
                    }
                }
            }

            _ => {
                 //don't do anything
            }
        }

        for child in node.children.borrow().iter() {
            walk(&mut link, child.clone());
        }
        //println!("{}", link.len());
        return link.to_vec();
    }


