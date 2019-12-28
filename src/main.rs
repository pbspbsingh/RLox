use std::env;
use std::fs;
use std::time::Instant;

use rlox::RLox;

fn main() {
    println!("Hello World, rlox version: {}", env!("CARGO_PKG_VERSION"));

    let mut rlox = RLox::default();
    match env::args().nth(1) {
        None => eprintln!("Usage: rlox <script file>"),
        Some(f) => {
            let src = fs::read_to_string(&f).unwrap_or_else(|e| panic!("Reading of '{}' failed: {}", f, e));

            let start = Instant::now();
            rlox.exec(src);
            println!("`rlox` finished in {} millis", start.elapsed().as_millis());
        }
    }
}
