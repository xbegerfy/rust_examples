use rust_examples::minigrep;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("failed to build config: {err}");
        process::exit(1);
    });
    if let Err(err) = minigrep::grep_file(&cfg) {
        eprintln!("faield to grep file content: {err}");
        process::exit(1);
    }
}
