/*
 * This file contains the structure of Crawler and its methods
 */

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper::Url;
use select::document::Document;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use function::*;
use error::*;

#[derive(Debug)]
pub struct Crawler {
	client: Client,
	bank: Arc<Mutex<Vec<Url>>>,
	queue: Arc<Mutex<VecDeque<Url>>>,
}

impl Crawler {
	pub fn new() -> Result<Crawler> {
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);
		let bank = Vec::new();
		let deque: VecDeque<Url> = VecDeque::new();

		let crawler = Crawler {
			client: client,
			bank: Arc::new(Mutex::new(bank)),
			queue: Arc::new(Mutex::new(deque)),
		};
		Ok(crawler)
	}

//////////==============================================\\\\\\\\\\\\
	pub fn new_shared(queue: Arc<Mutex<VecDeque<Url>>>,
					  bank: Arc<Mutex<Vec<Url>>>) -> Crawler {
		let mut crawler = Crawler::new().unwrap();
		crawler.queue = queue;
		crawler.bank = bank;
		crawler
	}
//////////==============================================\\\\\\\\\\\\

	pub fn bank(&self) -> Arc<Mutex<Vec<Url>>> {
		self.bank.clone()
	}

	pub fn queue(&self) -> Arc<Mutex<VecDeque<Url>>> {
		self.queue.clone()
	}

	pub fn check_empty(&self) -> bool {
		let queue = lock(&(self.queue)).unwrap();
		queue.is_empty()
	}

	pub fn len(&self) -> usize {
		let queue = lock(&self.queue).unwrap();
		queue.len()
	}

	pub fn pop(&self) -> Option<Url> {
		sync_pop_url(&self.queue)
	}

	pub fn push(&self, url: Url) -> Result<()> {
		sync_add_url(&self.queue, &self.bank, url)
	}

	pub fn parse_url(&self) {
		/*
		while !self.check_empty() {
			let length = self.len();

			let (tx, rx) = mpsc::channel::<T: Url>();
			for _ in 0..length {
				let tx = tx.clone();

				thread::spawn(move || {
					// let url = match self.pop() {
					// 	Some(url) => url,
					// 	None => continue,
					// };
					let url = self.pop().unwrap();
					let (url, body) = crawler(url).unwrap();
					let body = String::from_utf8_lossy(body.as_slice()).to_string();
					let doc = Document::from(body.as_str());
					let hrefs = scrap_href(&doc, "href");
					for href in hrefs {
						if href.starts_with('#') {
							continue;
						}
						let url = match convert_url(&url, &href) {
							Some(u) => u,
							None => continue,
						};
						self.push(url.clone());
						//println!("{:?}", url);
					}
				});
			}
			println!("{:?}", rx.recv().unwrap());
		}
		*/

		while !self.check_empty() {
			let url = match self.pop() {
				Some(url) => url,
				None => continue,
			};
			let (url, body) = crawler(url).unwrap();		
			let body = String::from_utf8_lossy(body.as_slice()).to_string();
			let doc = Document::from(body.as_str());
			let hrefs = scrap_href(&doc, "href");
			for href in hrefs {
				if href.starts_with('#') {
					continue;
				}
				let url = match convert_url(&url, &href) {
					Some(u) => u,
					None => continue,
				};
				self.push(url.clone());
				println!("{:?}", url);
			}
		}
	}
}