/*
 * This file contains the structure of CrawlerSet and its methods
 */

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper::Url;
use select::document::Document;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use crawler::*;
use function::*;
use error::*;

#[derive(Debug)]
pub struct CrawlerSet {
	set: Vec<Crawler>,
	bank: Arc<Mutex<Vec<Url>>>,
	queue: Arc<Mutex<VecDeque<Url>>>,
}

impl CrawlerSet {
	pub fn new() -> CrawlerSet {
		let set = Vec::new();
		let bank = Vec::new();
		let deque: VecDeque<Url> = VecDeque::new();

		CrawlerSet {
			set: set,
			bank: Arc::new(Mutex::new(bank)),
			queue: Arc::new(Mutex::new(deque)),
		}
	}

	pub fn create_clawers(&mut self, number: usize) {
		self.set.clear();
		for _ in 0..number {
			self.add_crawler();
		}
		if self.set.is_empty() {
			self.add_crawler();
		}
	}

	pub fn add_crawler(&mut self) {
		let bank = self.bank();
		let queue = self.queue();
		self.set.push(Crawler::new_shared(queue, bank));
	}

	pub fn crawl(&mut self) {
		self.set[0].parse_url()
	}

	pub fn crawl_recursive(&mut self) -> Result<Vec<Receiver<Url>>> {
		let mut rxs = Vec::new();
		while let Some(mut crawler) = self.set.pop() {
			let (tx, rx) = mpsc::channel();
			///////////++++++++++++++++++\\\\\\\\\\\\
			thread::spawn(move || crawler.parse_url());
			///////////++++++++++++++++++\\\\\\\\\\\\
			rxs.push(rx)
		}

		Ok(rxs)
	}

	pub fn add_url(&mut self, url: Url) -> Result<()> {
		sync_add_url(&self.queue, &self.bank, url)
	}

	pub fn bank(&self) -> Arc<Mutex<Vec<Url>>> {
		self.bank.clone()
	}

	pub fn queue(&self) -> Arc<Mutex<VecDeque<Url>>> {
		self.queue.clone()
	}
}