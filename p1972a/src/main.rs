use std::io::{self, BufRead};
use contest_proposal::contests;

fn main() {
    let stdin = io::stdin();
    contests(stdin.lock().lines());
}
