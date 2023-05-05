use crate::modules::args::Args;

pub struct Banner;
impl Banner {
    pub fn print(args: &Args) {
        println!();
        println!("URL: {}", args.url);
        println!("Max Concurrency: {}", args.max_concurrency);
        println!("Wordlist Path: {}", args.wordlist_path);
        println!();
    }
}
