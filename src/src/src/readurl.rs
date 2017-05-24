extern crate hyper;

use std::env;
use self::hyper::Url;

pub fn readurl() -> Url {       //read the input argument as an absolute url
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