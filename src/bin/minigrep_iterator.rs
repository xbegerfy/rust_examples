use rust_examples::minigrep_iterator;
use std::{env, process};

fn main() {
    let args = env::args();
    let cfg = minigrep_iterator::Config::build(args).unwrap_or_else(|err| {
        eprintln!("failed to build config: {err}");
        process::exit(1);
    });
    if let Err(err) = minigrep_iterator::grep_file(&cfg) {
        eprintln!("faield to grep file content: {err}");
        process::exit(1);
    }
}
