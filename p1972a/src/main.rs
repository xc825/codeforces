use std::io::{self, BufRead};
use contest_proposal::contests;

fn main() {
    let stdin = io::stdin();
    contests(stdin.lock().lines());
}

//"6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n";
//"6\n4 5 6 7 8 9\n1 2 3 4 5 6\n";
// echo -e "2\n6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n6\n4 5 6 7 8 9\n1 2 3 4 5 6\n" | cargo run