use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::{stdout, Write};
use crossterm::{
    execute,
    cursor,
    terminal::{Clear, ClearType},
    style::Stylize
};

use crate::modules::request::RequestHandler;


pub struct Executor;
impl Executor {
    pub async fn run(request_handler: RequestHandler, max_concurrent_requests: usize, wordlist: Vec<String>) {
        let requests_sendt = Arc::new(Mutex::new(0));
        let stdout = Arc::new(Mutex::new(stdout()));
        for word in wordlist {
            let stdout_clone = stdout.clone();
            let request = request_handler.clone();
            let counter = requests_sendt.clone();

            let fut = request.send(word.clone().to_string());

            tokio::spawn(async move {
            let word = word.clone();
                match fut.await {
                    Ok(res) => match res {
                        ref res if res != "404 Not Found" => {
                            // println!("\n/{}", {&word});
                            let mut stdout_lock = stdout_clone.lock().await;

                            println!();
                            execute!(stdout_lock, cursor::MoveUp(1)).unwrap();
                            execute!(stdout_lock, Clear(ClearType::CurrentLine)).unwrap();

                            color_print("/", word, res);
                            
                            let mut counter = counter.lock().await;
                            *counter += 1; 
                            print_counter(*counter);
                        }
                        _ => {
                            let mut counter = counter.lock().await;
                            *counter += 1; 
                            print_counter(*counter);
                        },
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            });
        }
        loop {
            tokio::task::yield_now().await;
            let counter = requests_sendt.lock().await;
            if request_handler.semaphore.available_permits() == max_concurrent_requests { break; }
            drop(counter);
        }
    }
}
fn print_counter(counter: usize) {
    let to_print = format!("[{}] {} {}\r", "?".blue(), counter.to_string().green(), "Requests sent...");
    print!("{to_print}");
    std::io::stdout().flush().unwrap();
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // Move cursor to top-left corner and clear the console
    stdout().flush().unwrap(); // Ensure that the output is immediately visible
}

fn color_print(prompt: &str, to_print: String, status: &String) {
    println!("[{}] {}{} {}", "+".blue(), prompt, to_print.green(), status);
}
