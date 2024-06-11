use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let cole = stdout();
    let message = String::from("Sup crusters");
    let width = message.chars().count();

    let mut writer = BufWriter::new(cole.lock());
    say(&message, width, &mut writer).unwrap();
}
