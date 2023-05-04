use std::error::Error;
use std::sync::Arc;
use tokio::sync::Semaphore;
use reqwest::Client;

pub mod modules;
use crate::modules::wordlist::Wordlist;
use crate::modules::request::make_request;



async fn run(url: String, max_concurrent_requests: usize) {

    let sem = Arc::new(Semaphore::new(max_concurrent_requests));

    // We use a client to reuse connections.
    let client = Client::new();

    for i in 1..10000 {
        let fut = make_request(url.clone(), client.clone(), sem.clone());
        tokio::spawn(async move {
            match fut.await {
                Ok(ok) => println!("{}", i),
                Err(e) => println!("{}", e)
            }
        });
    }

    loop {
        tokio::task::yield_now().await;
        if sem.available_permits() == max_concurrent_requests { break; }
    }
}

#[tokio::main]
async fn main() {
    let url = String::from("http://127.0.0.1:5000");
    let max_concurrent_requests: usize = 10;
    let wordlist = Wordlist::from_path("/home/z4/hackin/wordlists/web/directory-list-lowercase-2.3-medium.txt".to_string());
    run(url, max_concurrent_requests).await;
}
