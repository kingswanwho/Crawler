/*
 * This file contains the basic functions
 */

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper::Url;
use select::document::Document;
use select::predicate::Attr;
use std::env;
use std::io::Read;
use hyper::client::IntoUrl;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use error::*;

pub fn lock<T>(mutex: &Arc<Mutex<T>>) -> Result<MutexGuard<T>> {
	match mutex.lock() {
		Ok(t) => Ok(t),
		Err(e) => bail!(ErrorKind::PoisonError(e.to_string())),
	}
}

pub fn sync_pop_url(queue: &Arc<Mutex<VecDeque<Url>>>) -> Option<Url> {
	let mut queue = lock(queue).unwrap();
	queue.pop_front()
}

pub fn sync_add_url(queue: &Arc<Mutex<VecDeque<Url>>>, bank: &Arc<Mutex<Vec<Url>>>, url: Url) -> Result<()> {
	let mut queue = lock(queue)?;
	let mut bank = lock(bank)?;

	if !bank.contains(&url) {
		bank.push(url.clone());
		queue.push_back(url.clone());
	}

	Ok(())
}

pub fn scrap_href(doc: &Document, attr: &str) -> Vec<String> {
	let mut attrs = Vec::new();
	let nodes = doc.find(Attr(attr, ()));		//find<P: Predicate>(&self, predicate: P)Returns a Selection 
												//containing nodes passing the givrn predicate p
	for node in nodes.iter() {
		let attr = match node.attr("href") {
			Some(a) => a.to_string(),
			None => continue,
		};
		attrs.push(attr);
	}
	attrs
}

pub fn readurl() -> Url {       		//read the input argument as an absolute url
	match env::args().nth(1) {
    	Some(url_str) => {
    		match Url::parse(&url_str) {
    			Ok(url) => {   				
    				url
    			}
    			Err(_) => panic!("Panic!: The argument is not an url!"),
    		}
    	}
    	None => panic!("Panic!: It lacks an input argument!"),
    }
}

pub fn convert_url(url: &Url, href: &str) -> Option<Url> {
    let url = if href.starts_with("//") {
        let scheme = url.scheme();              //Return the scheme of this URL, lower-cased, as an ASCII string 
                                                //without the ':' delimiter and following string.
        match format!("{}:{}", scheme, href).into_url() {       //Consumes the object, trying to return a Url.
            Ok(u) => u,
            _ => return None,
        }
    } else if href.starts_with("http") {
        match href.into_url() {
            Ok(u) => u,
            _ => return None,
        }
    } else if href.starts_with('/') {
        let mut url = url.clone();
        url.set_path(href);                     //Change this URL's path
        url
    } else if href.starts_with("javascript") {
        return None;
    } else {
        let path = url.path();
        if path.ends_with(href) {
            return None;
        }
        let mut url = url.clone();
        let href = format_url(format!("{}/{}", url, href));
        url.set_path(&href);
        url
    };
    Some(url)
}

pub fn format_url<S: AsRef<str>>(url: S) -> String {
	let mut result = String::new();
	let url = url.as_ref();
	let mut last_char = ' ';
	for ch in url.chars() {
		if ch == '/' && last_char == '/' {
			continue;
		}
		result.push(ch);
		last_char = ch;
	}
	result
}

pub fn crawler(url: Url) -> Result<(Url, Vec<u8>)> {
	let ssl = NativeTlsClient::new().unwrap();
	let connector = HttpsConnector::new(ssl);
	let client = Client::with_connector(connector);

	let mut response = client.get(url.clone()).send().unwrap();
	let mut buf = Vec::new();
	let body = match response.read_to_end(&mut buf) {
		Ok(_) => buf,
		Err(e) => bail!(e),
	};
	Ok((url, body))
}
