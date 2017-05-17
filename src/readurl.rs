extern crate hyper;

use std::env;
use self::hyper::Url;

pub fn readurl () -> Url {
    let url_str = env::args().nth(1).unwrap();
    let url = Url::parse(&url_str).unwrap();
    return url;
}