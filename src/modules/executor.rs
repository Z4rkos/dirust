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
        let wordlist_len = wordlist.len();
        for word in wordlist {
            let stdout_clone = stdout.clone();
            let request = request_handler.clone();
            let counter = requests_sendt.clone();

            let fut = request.send(word.clone().to_string());

            tokio::spawn(async move {
            let word = word.clone();
                match fut.await {
                    Ok(res) => match res {
                        ref res if res != "404" => {
                            // println!("\n/{}", {&word});
                            let mut stdout_lock = stdout_clone.lock().await;

                            println!();
                            execute!(stdout_lock, cursor::MoveUp(1)).unwrap();
                            execute!(stdout_lock, Clear(ClearType::CurrentLine)).unwrap();

                            color_print("/", word, &color_code(res));
                            
                            let mut counter = counter.lock().await;
                            *counter += 1; 
                            print_counter(*counter, wordlist_len);
                        }
                        _ => {
                            let mut counter = counter.lock().await;
                            *counter += 1; 
                            print_counter(*counter, wordlist_len);
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
// All of the following functions should either be moved into the struct impl, or to it's own file.
fn print_counter(counter: usize, wordlist_len: usize) {
    let to_print = format!("[{}] {}/{}\r", "?".blue(), counter, wordlist_len);
    print!("{to_print}");

    // Makes the output look nicer, but uses aloooot of cpu resources as it runs so often.
    // std::io::stdout().flush().unwrap();
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // Move cursor to top-left corner and clear the console
    stdout().flush().unwrap(); // Ensure that the output is immediately visible
}

fn color_print(prompt: &str, to_print: String, status: &String) {
    let to_print_len = to_print.len();
    println!("[{}] {}{}{}{}{}", "+".blue(), prompt, to_print.blue(), " ".repeat(20 - to_print_len), status, " ".repeat(10));
}

// color_code doesn't need to make these everytime it runs. Should make this a struct.
const GOOD_RANGE: std::ops::Range<u16> = 200..300;
const REDIRECT_RANGE: std::ops::Range<u16> = 300..400;
const BAD_RANGE: std::ops::Range<u16> = 400..506;

fn color_code(code: &str) -> String {
    match code.parse::<u16>() {
        Ok(status_code) if GOOD_RANGE.contains(&status_code) => status_code.to_string().green().to_string(),
        Ok(status_code) if REDIRECT_RANGE.contains(&status_code) => status_code.to_string().blue().to_string(),
        Ok(status_code) if BAD_RANGE.contains(&status_code) => status_code.to_string().red().to_string(),
        Ok(status_code) => status_code.to_string().white().to_string(),
        Err(_) => code.to_string().white().to_string()
    }
}
