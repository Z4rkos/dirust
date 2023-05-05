use crate::modules::request::RequestHandler;
use std::sync::Arc;
use tokio::sync::Mutex;


pub struct Executor;
impl Executor {
    pub async fn run(request_handler: RequestHandler, max_concurrent_requests: usize, wordlist: Vec<String>) {
        let requests_sendt = Arc::new(Mutex::new(0));
        // let mut test: Vec<String> = Vec::new();
        for word in wordlist {
            let request = request_handler.clone();
            let counter = requests_sendt.clone();

            let fut = request.send(word.clone());
            tokio::spawn(async move {
                match fut.await {
                    Ok(res) => match res {
                        ref res if res == "200 OK" => {
                            println!("\n/{}", {word});
                            // let stuff = format!("/{}\n", word);
                            // test.push(String::from(stuff));
                            let mut counter = counter.lock().await; // Acquire a lock on the counter
                            *counter += 1; // Increment the counter
                        }
                        _ => {
                            let mut counter = counter.lock().await; // Acquire a lock on the counter
                            *counter += 1; // Increment the counter
                            print_counter(*counter);
                        },
                    },
                    Err(e) => println!("{}", e)
                }
            });
        }
        loop {
            tokio::task::yield_now().await;
            let counter = requests_sendt.lock().await; // Acquire a lock on the counter
            if request_handler.semaphore.available_permits() == max_concurrent_requests { break; }
            drop(counter);
        }
    }
}
fn print_counter(counter: usize) {
    use std::io::Write;
    print!("{} requests sent\r", counter);
    std::io::stdout().flush().unwrap();
}
