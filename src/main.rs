use std::env;

use minigrep;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let s = minigrep::run(&args);
    println!("{s}");
}
