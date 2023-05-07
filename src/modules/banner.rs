use crossterm::style::Stylize;

use crate::modules::args::Args;


pub struct Banner;
impl Banner {
    pub fn print(args: &Args) {
        println!();
        // println!("URL: {}", args.url);
        color_print("URL: ", args.url.to_string());
        color_print("Max Concurrency: ", args.max_concurrency.to_string());
        color_print("Wordlist Path: ", args.wordlist_path.to_string());
        println!();
    }
}

fn color_print(prompt: &str, to_print: String) {
    println!("[{}] {}{}", "?".blue(), prompt, to_print.blue());
}
