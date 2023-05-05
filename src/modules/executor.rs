use crate::modules::request::RequestHandler;
use std::sync::Arc;
use tokio::sync::Mutex;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};


pub struct Executor;
impl Executor {
    pub async fn run(request_handler: RequestHandler, max_concurrent_requests: usize, wordlist: Vec<String>) {
        let requests_sendt = Arc::new(Mutex::new(0));
        let mut stdout = Arc::new(Mutex::new(stdout()));
        let word_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        for word in wordlist {
            let mut stdout_clone = stdout.clone();

            let request = request_handler.clone();
            let counter = requests_sendt.clone();
            let word_vec = word_vec.clone();

            let fut = request.send(word.clone().to_string());
            tokio::spawn(async move {
            let word = word.clone();
                match fut.await {
                    Ok(res) => match res {
                        ref res if res == "200 OK" => {
                            // println!("\n/{}", {&word});
                            let stuff = format!("/{}", word);
                            let mut locked_vec = word_vec.lock().await;
                            let mut stdout_lock = stdout_clone.lock().await;

                            // clear_console();
                            locked_vec.push(stuff);
                            execute!(stdout_lock, Clear(ClearType::All)).unwrap();
                            for line in locked_vec.iter() {
                                println!("{}", line);
                            }
                            
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

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // Move cursor to top-left corner and clear the console
    stdout().flush().unwrap(); // Ensure that the output is immediately visible
}
