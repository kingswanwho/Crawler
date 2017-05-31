/*
 * Final Project: Concurrent Web Crawler in Rust
 * Author: Mingyang Li, Jingchuan Hu
 *
 * Code Explanation:
 * In this code, we input an argument of the begining url of the target website
 * We use the BFS algorithm and concurrent method to find and crawl all urls.
 * First we push the input url into the dequeue, then use one thread to parse 
 * (pop it out from the dequeue) this url and get all sub-urls from it and push 
 * them into the dequeue.
 * Then recursively use multiple threads to parse (pop them out) these urls 
 * which are in the dequeue and push all the parsed sub-urls into the dequeue 
 * until the dequeue is empty.
 */

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
use crawler_set::*;

fn main() {
    let mut crawlers = CrawlerSet::new();
    let url = readurl();
    crawlers.start_crawl(url);
}
