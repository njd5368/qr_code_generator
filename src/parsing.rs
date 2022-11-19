use clap::Parser;

#[derive(Parser,Debug)]
#[command(author,version,about, long_about = None)]
pub struct Args {
    // Takes in the first argument <url> no -- or - needed
    #[arg()]
    pub url: String,
    
    // takes in the opiton of what format to use
    #[arg(short, long, required = false)]
    pub format: Option<u32>,
    
    #[arg(short, long, required = false)]
    pub output: Option<String>,
}

pub fn check_format(fmt: Option<u32>) -> u32{
    match fmt {
        Some(x) => x,
        None => 21
    }
}

