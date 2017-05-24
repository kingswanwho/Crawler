use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use hyper::error::ParseError;
use hyper::client::IntoUrl;
use hyper::Url;

use crawler::Indexer;

pub fn lock<T>(mutex: &Arc<Mutex<T>>) -> Result<MutexGuard<T>, ParseError> {
	match mutex.lock() {
		Ok(t) => Ok(t),
		//Err(e) => ParseError::ErrorKind::PoisonError(e.to_string()),
		Err(_) => ParseError::EmptyHost,
	}
}

pub fn add_to_queue<U: IntoUrl>(indexer: &Arc<Mutex<Indexer>>,
                            	queue: &Arc<Mutex<VecDeque<Url>>>,
                            	url: U) -> Result<()> {
	let url = url.into_url()?;
	let mut queue = lock(queue)?;
	if !queue.contains(&url) && !lock(indexer)?.is_indexed(&url) {
		queue.push_back(url);
	}
	Ok(())
}

pub fn queue_items(queue: &Arc<Mutex<VecDeque<Url>>>) -> Result<VecDeque<Url>> {
	let queue = lock(queue)?;
	Ok(queue.clone())
}

pub fn is_queue_empty(queue: &Arc<Mutex<VecDeque<Url>>>) -> bool {
	let queue = match lock(queue) {
		Ok(q) => q,
		Err(_) => return true,
	};
	queue.is_empty()
}

pub fn pop_queue(queue: &Arc<Mutex<VecDeque<Url>>>) -> Result<Url> {
	let mut queue = lock(queue)?;
	let url = queue.pop_front();
	match url {
		Some(u) => Ok(u),
		None => {},
	}
}

pub fn get_running(running: &Arc<AtomicUsize>) -> usize {		//Get number of slave running
	running.load(Ordering::SeqCst)		//load(E): loads a value from the atomic integer
}

pub fn add_running(running: &Arc<AtomicUsize>) {	//Add one to running count
	running.fetch_add(1, Ordering::SeqCst);			//Add to the current value, returning the previous value
}

pub fn remove_running(running: &Arc<AtomicUsize>) {	//Remove one to running count
	if get_running(running) == 0 {
		return;
	}
	running.fetch_sub(1, Ordering::SeqCst);			//Subtract from the current value, returning the previous value
}

pub fn get_stop(stop: &Arc<AtomicBool>) -> bool {	//Get the stop value
	stop.load(Ordering::Relaxed)					//Loads a value from the bool
}

pub fn set_stop(stop_async: &Arc<AtomicBool>, stop: bool) {	//Set the stop value
	stop_async.store(stop, Ordering::Relaxed);
}