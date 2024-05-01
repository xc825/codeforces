use std::io::{BufRead};

pub fn contests<R: BufRead>(reader: R) where R: Clone {
    let mut reader_clone = reader.clone();
    let mut first_line = String::new();
    reader_clone.read_line(&mut first_line).unwrap();
    let n: i32 = first_line.trim().parse().unwrap();
    println!("{}", n);
    for _ in 0..n {
        println!("{}", contest_proposal(reader.clone()));
    }
}

pub fn contest_proposal<R: BufRead>(reader: R) -> i32 {
    let mut lines = reader.lines();

    let n = lines.next().unwrap().unwrap().trim().parse().unwrap();
    println!("input:{}", n);

    let numbers_a: Vec<i32> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("input:{:?}", numbers_a);

    let numbers_b: Vec<i32> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("input:{:?}", numbers_b);

    let mut i_a: usize = 0;
    for i_b in 0..n {
        if numbers_b[i_b] >= numbers_a[i_a] {
            i_a += 1;
        }
    }
    (n - i_a) as i32
}

#[cfg(test)]
mod tests {
use std::io::Cursor;
use super::contest_proposal;

    #[test]
    fn test1() {
        let data = b"6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n";
        let cursor = Cursor::new(&data[..]);
        assert_eq!(contest_proposal(cursor), 2);
    }

    #[test]
    fn test2() {
        let data = b"6\n4 5 6 7 8 9\n1 2 3 4 5 6\n";
        let cursor = Cursor::new(&data[..]);
        assert_eq!(contest_proposal(cursor), 3);
    }
}
