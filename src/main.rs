#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_native_tls;
extern crate select;

mod error;
mod function;
mod crawler;
mod crawler_set;

use function::*;
use crawler::*;
use crawler_set::*;
use std::io::Read;

fn main() {
    let crawler = Crawler::new().unwrap();
    let url = readurl();
    crawler.push(url);
    crawler.parse_url();

    let mut crawlers = CrawlerSet::new();
    let url = readurl();
    crawlers.add_url(url);
    crawlers.crawl_recursive();
}
