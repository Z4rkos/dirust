use std::sync::Arc;
use tokio::sync::Semaphore;

pub mod modules;
use crate::modules::wordlist::Wordlist;
use crate::modules::request::{RequestBuilder, Request};



async fn run(request: Request, max_concurrent_requests: usize, wordlist: Vec<String>) {
    for word in wordlist {
        let request = request.clone();

        let fut = request.send(word.clone());
        tokio::spawn(async move {
            match fut.await {
                Ok(res) => match res {
                    ref res if res == "200 OK" => {
                        println!("/{}", {word});
                    }
                    _ => ()
                },
                Err(e) => println!("{}", e)
            }
        });
    }

    loop {
        tokio::task::yield_now().await;
        if request.semaphore.available_permits() == max_concurrent_requests { break; }
    }
}

#[tokio::main]
async fn main() {
    let url = String::from("http://127.0.0.1:5000");
    let max_concurrent_requests: usize = 1;
    let wordlist = Wordlist::from_path("/home/z4/hackin/wordlists/web/directory-list-lowercase-2.3-medium.txt".to_string()).unwrap();

    // let request = Request::new(url, client, sem.clone());
    let request = RequestBuilder::new()
        .url(url)
        .semaphore(Arc::new(Semaphore::new(max_concurrent_requests)))
        .build();

    println!("{}\n{}", request.url, request.semaphore.available_permits());
    run(request, max_concurrent_requests, wordlist).await;
}
