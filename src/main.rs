use std::sync::Arc;
use tokio::sync::Semaphore;

pub mod modules;
use crate::modules::args::get_args;
use crate::modules::wordlist::Wordlist;
use crate::modules::request::RequestBuilder;
use crate::modules::executor::Executor;


#[tokio::main]
async fn main() {
    let args = get_args();

    let request = RequestBuilder::new()
        .url(args.url)
        .semaphore(Arc::new(Semaphore::new(args.max_concurrency)))
        .build();

    let wordlist = Wordlist::from_path(args.wordlist_path).expect("Could not open wordlist.");
    Executor::run(request, args.max_concurrency, wordlist).await;
}
