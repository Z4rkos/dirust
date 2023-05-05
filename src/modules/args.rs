use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'u', long = "url")]
    pub url: String,

    #[arg(short = 'w', long = "wordlist")]
    pub wordlist_path: String,

    #[arg(short = 'm', long = "max_concurrency", default_value_t = 10)]
    pub max_concurrency: usize,
}

// This is literally just so I don't have to do "use::clap::Parser;" in main
pub fn get_args() -> Args {
    Args::parse()
}
