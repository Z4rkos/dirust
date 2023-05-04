use std::sync::Arc;
use tokio::sync::Semaphore;
use reqwest::Client;

pub mod modules;
use crate::modules::wordlist::Wordlist;
use crate::modules::request::Request;



async fn run(sem: Arc<Semaphore>, request: Request, max_concurrent_requests: usize, wordlist: Vec<String>) {
    for word in wordlist {
        let mut request = request.clone();
        request.url = format!("{}/{}", request.url, word);

        let fut = request.clone().send();
        tokio::spawn(async move {
            match fut.await {
                Ok(res) => match res {
                    ref res if res == "200 OK" => {
                        println!("/{}", {word});
                    },
                    _ => ()
                },
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
    let max_concurrent_requests: usize = 30;
    let wordlist = Wordlist::from_path("/home/z4/hackin/wordlists/web/directory-list-lowercase-2.3-medium.txt".to_string()).unwrap();

    let sem = Arc::new(Semaphore::new(max_concurrent_requests));
    let client = Client::new();

    let request = Request::new(url, client, sem.clone());
    run(sem, request, max_concurrent_requests, wordlist).await;
}
