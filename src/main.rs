use std::sync::Arc;
use reqwest::Client;
use tokio::sync::Semaphore;

pub mod modules;
use crate::modules::{
    args::get_args,
    banner::Banner,
    wordlist::Wordlist,
    request::RequestHandlerBuilder,
    executor::Executor,
};


#[tokio::main]
async fn main() {
    let args = get_args();
    Banner::print(&args);

    let request = RequestHandlerBuilder::new()
        .url(args.url)
        .client(Client::new())
        .semaphore(Arc::new(Semaphore::new(args.max_concurrency)))
        .build();

    let wordlist = Wordlist::from_path(args.wordlist_path).expect("Could not open wordlist.");
    // let wordlist = Wordlist::from_range(1..10000);
    Executor::run(request, args.max_concurrency, wordlist).await;
    println!("\nFinished!")
}
