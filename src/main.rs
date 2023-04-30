use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello - Rustaceans!");
    let width = message.chars().count();
//TODO

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_str(), width, &mut writer).unwrap();
}