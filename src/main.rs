use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use markitdown::MarkItDown;

/// Simple program that will output the files in the path in markdown syntax
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to the content we want to copy
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();

    let mut ctx = ClipboardContext::new().unwrap();

    let content = MarkItDown::get_content(&args.path);

    ctx.set_contents(content).unwrap();

    println!("Content has been copied to your clipboard");
}
