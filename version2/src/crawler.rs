extern crate error_chain;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use hyper::Url;
use hyper::client::Client;
use hyper::client::IntoUrl;
use hyper::error::ParseError;

pub struct Crawler {				//Add settings to go deeper or else
	slaves: Vec<CrawlerSlave>,
	indexer: Arc<Mutex<Indexer>>,
	queue: Arc<Mutex<VecDeque<Url>>>,
	running: Arc<AtomicUsize>,
	stop: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct Site {
	url: Url,
	subs_url: Vec<Url>,
}

impl Site {
	pub fn new<U: IntoUrl>(url: U) -> Result<Site, ParseError> {        //Create a new instance of site
        let mut url = url.into_url()?;
        let sub_url = url.clone();
        url.set_path("");       //Append this URL's path

        let mut subs_url = Vec::new();
        if sub_url != url {
            subs_url.push(sub_url);
        }
        Ok(Site {
            url: url,
            subs_url: subs_url,
        })
    }

    pub fn add_sub_url<U: IntoUrl>(&mut self, sub_url: U) {
    	let sub_url = match sub_url.into_url() {
    		Ok(u) => u,
    		Err(_) => return,
    	};
    	if self.url.domain() == sub_url.domain() {	//If sub_url's host is same as the url's domain
    		self.subs_url.push(sub_url);			//then push the sub_url into the subs_url
    	}
    }

    pub fn add_subs_url<U: IntoUrl>(&mut self, subs_url: Vec<U>) {
    	for sub_url in subs_url {
    		self.add_sub_url(sub_url);
    	}
    }

    pub fn contains_url(&self, url: &Url) -> bool {
    	if url.domain() != self.url.domain() {
    		return false;
    	}

    	let url_path = url.path();
    	if url_path == self.url.path() {
    		return true;
    	}
    	for sub_url in &self.subs_url {
    		if sub_url.path() == url_path {
    			return true;
    		}
    	}
    	false
    }

    pub fn si_same_host(&self, url: &Url) -> bool {
    	self.url.host_str() == url.host_str()
    }

    pub fn get_url(&self) -> &Url {
    	&self.url
    }

    pub fn get_subs_url(&self) -> &Vec<Url> {
    	&self.subs_url
    }

    pub fn get_subs_url_str(&self) -> Vec<&str> {
    	self.subs_url.iter().map(|u| u.as_str()).collect()
    }
}

#[derive(Debug)]
pub struct Indexer {
	sites: Vec<Site>,
}

impl Indexer {
	pub fn new() -> Indexer {
		Indexer {
			sites: Vec::new()
		}
	}

	// pub fn add_url<U: IntoUrl>(&mut self, url: U) -> Result<()> {
	// 	let url = url.into_url()?;
	// 	for site in &mut self.sites {
	// 		if site.contains_url(&url) {
	// 			//bail!(ErrorKind::Msg("xxxx"));
	// 		}
	// 		if site.is_same_host(&url) {
	// 			site.add_sub_url(url);
	// 			return Ok(());
	// 		}
	// 	}

	// 	self.sites.push(Site::new(url)?);
	// 	Ok(())
	// }

	pub fn get_sites(&self) -> &Vec<Site> {
		&self.sites
	}

	pub fn get_all_urls(&self) -> Vec<&Url> {
		let mut vec = Vec::new();
		for site in &self.sites {
			vec.push(site.get_url());
			vec.extend(site.get_subs_url());
		}
		vec
	}

	pub fn get_all_main_urls(&self) -> Vec<&Url> {
		self.sites.iter().map(|s| s.get_url()).collect()
	}

	pub fn get_all_subs_urls(&self) -> Vec<&Url> {
        let mut sub_url_vec = Vec::new();
        for site in &self.sites {
            sub_url_vec.extend(site.get_subs_url());
        }
        sub_url_vec
    }

    pub fn is_indexed(&self, url: &Url) -> bool {
    	for site in &self.sites {
    		if site.contains_url(url) {
    			return true;
    		}
    	}
    	false
    }
}

#[derive(Debug)]
pub struct CrawlerSlave {
	client: Client,
	indexer: Arc<Mutex<Indexer>>,
	queue: Arc<Mutex<VecDeque<Url>>>,
	running: Arc<AtomicUsize>,
	stop: Arc<AtomicBool>,
}
/*
impl CrawlerSlave {
	pub fn new() -> CrawlerSlave {
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		CrawlerSlave {
			client: Client::with_connctor(connector),
			indexer: Arc::new(Mutex::new(Indexer::new())),
			queue: Arc::new(Mutex::new(VecDeque::new())),
			running: Arc::new(AtomicUsize::new(0)),
			stop: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn new_shared(indexer: Arc<Mutex<Indexer>>,
					  queue: Arc<Mutex<VecDeque<Url>>>,
					  running: Arc<AtomicUsize>,
					  stop: Arc<AtomicBool>) -> CrawlerSlave {
		let mut crawler_slave = CrawlerSlave::new();
		crawler_slave.indexer = indexer;
		crawler_slave.queue = queue;
		crawler_slave.running = running;
		crawler_slave.stop = stop;
		crawler_slave
	}

	pub fn crawl(&mut self) -> Result<(Url, Vec<u8>)> {
		let url = sync::pop_queue(&self.queue)
	}
}
*/


