use std::io::Result;
use std::iter::Iterator;

pub fn contests<I, S>(mut lines: I)
where
    I: Iterator<Item = Result<S>>,
    S: AsRef<str>,
{
    let n = lines.next().unwrap().unwrap().as_ref().parse::<i32>().unwrap();

    for _ in 0..n {
        println!("{}", contest_proposal(&mut lines));
    }
}

pub fn contest_proposal<I, S>(mut lines: I) -> i32
where
    I: Iterator<Item = Result<S>>,
    S: AsRef<str>,
{
    let n: u32 = lines.next().unwrap().unwrap().as_ref().parse::<u32>().unwrap();

    let numbers_a = lines.next().unwrap().unwrap().as_ref().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let numbers_b = lines.next().unwrap().unwrap().as_ref().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut i_a: usize = 0;
    for i_b in 0..n as usize{
        if numbers_b[i_b] >= numbers_a[i_a] {
            i_a += 1;
        }
    }
    (n as usize - i_a) as i32
}


#[cfg(test)]
mod tests {
use super::contest_proposal;
use std::io::{BufRead, BufReader};

    #[test]
    fn test1() {
        let data = b"6\n1000 1400 2000 2000 2200 2700\n800 1200 1500 1800 2200 3000\n";
        let reader = BufReader::new(&data[..]);
        let result = contest_proposal(reader.lines());
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let data = b"6\n4 5 6 7 8 9\n1 2 3 4 5 6\n";
        let reader = BufReader::new(&data[..]);
        let result = contest_proposal(reader.lines());
        assert_eq!(result, 3);
    }
}
