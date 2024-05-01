use std::io::{self, Read, Cursor};
use contest_proposal::contests;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer).unwrap();
    let cursor = Cursor::new(buffer);
    contests(cursor);
}

//"6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n";
//"6\n4 5 6 7 8 9\n1 2 3 4 5 6\n";
// echo -e "2\n6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n6\n4 5 6 7 8 9\n1 2 3 4 5 6\n" | cargo run