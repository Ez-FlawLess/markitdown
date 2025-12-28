#![deny(clippy::unwrap_used)]

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

    let mut ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to initialize clipboard: {}", e);
            std::process::exit(1);
        }
    };

    let content = MarkItDown::get_content(&args.path);

    if let Err(e) = ctx.set_contents(content) {
        eprintln!("Failed to set clipboard contents: {}", e);
        std::process::exit(1);
    }

    println!("Content has been copied to your clipboard");
}
