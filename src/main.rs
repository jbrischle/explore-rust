use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let mut writer = BufWriter::new(stdout().lock());
    say("hello world".as_bytes(), 5, &mut writer).unwrap()
}
