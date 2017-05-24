#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_native_tls;
extern crate select;

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper::Url;
use select::document::Document;
use select::predicate::Attr;
// use html5ever::parse_document;
// use html5ever::rcdom::{NodeData, RcDom, Handle};
// use html5ever::tendril::TendrilSink;

mod readurl;
mod crawler;
mod sync;
//mod error;
mod test;

fn main() {
	let mut url = readurl::readurl();
	println!("The read url result is {:?}", url);

	let ssl = NativeTlsClient::new().unwrap();	//Returns a NativeTlsClient with a default configuration

	let connector = HttpsConnector::new(ssl);	//Create a new connector using the provide SSL implementation

	let client = Client::with_connector(connector);	//Create a new client with a specific conncetor
	//println!("{:?}", client);

	let mut res = client.get(url).send().unwrap();	//get<U: IntoUrl>(&self, url: U) -> RequestBuilder
													//Build a Get request
													//send(self) -> Result<Response>
													//Execute this request and receive a Response back
	//println!("{:?}", res);

	// let dom = parse_document(RcDom::default(), Default::default())
	// 		  .from_utf8().read_from(&mut res).unwrap();	//parse_document<Sink>(sink: Sink, opts: ParseOpts) -> Parser<Sink>
	// 		  												//where Sink: TreeSink
	// 		  												//Parse an HTML document
	// 		  												//The returned value implements tendril::TendrilSink so that Unicode
	// 		  												//input may be provided incrementally, or all at once with the one method.
	// println!("{:?}", dom);

	
}


fn scrap_attr(doc: &Document, attr: &str) -> Vec<String> {
	let mut attrs = Vec::new();
	let nodes = doc.find(Attr(attr, ()));

	for node in nodes.iter() {
		let attr = match node.attr("href") {
			Some(a) => a.to_string(),
			None => continue,
		};
		attrs.push(attr);
	}
	attrs
}

error_chain!{
    foreign_links {
        Io(::std::io::Error);
        Hyper(::hyper::Error);
        Url(::hyper::error::ParseError);
    }

    errors {
        UrlAlreadyIndexed {
            description("Could not add url because it is already indexed")
            display("Url is already indexed")
        }
        PoisonError(e: String) {
            description(e)
            display("{}", e)
        }
        QueueEmpty {
            description("Queue has no item in it")
            display("Queue has no item in it")
        }
    }
}

