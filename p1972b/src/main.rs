use std::io::{self, BufRead};
use std::io::Result;
use std::iter::Iterator;

pub fn coin_games<I, S>(mut lines: I)
where
    I: Iterator<Item = Result<S>>,
    S: AsRef<str>,
{
    let n = lines.next().unwrap().unwrap().as_ref().parse::<i32>().unwrap();

    for _ in 0..n {
        println!("{}", game(&mut lines));
    }
}

//removing the middle coin changes the number of heads-up coins
//from even to odd or odd to even
pub fn game<I, S>(mut lines: I) -> String
where
    I: Iterator<Item = Result<S>>,
    S: AsRef<str>,
{
    let _n = lines.next().unwrap().unwrap().as_ref().parse::<u32>().unwrap();
    let coins = lines.next().unwrap().unwrap().as_ref().bytes().collect::<Vec<u8>>();
    //println!("{:?}", n);
    //println!("{:?}", String::from_utf8(coins).unwrap());
    if coins.iter().filter(|&c| *c == b'U').count() % 2 == 0 {
        "NO".to_string()
    } else {
        "YES".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    coin_games(stdin.lock().lines());
}

#[cfg(test)]
mod tests {
    use super::game;
    use std::io::{BufRead, BufReader};
    
        #[test]
        fn test1() {
            let data = b"5\nUUDUD\n";
            let reader = BufReader::new(&data[..]);
            let result = game(reader.lines());
            assert_eq!(result, "YES".to_string());
        }
    
        #[test]
        fn test2() {
            let data = b"5\nUDDUD\n";
            let reader = BufReader::new(&data[..]);
            let result = game(reader.lines());
            assert_eq!(result, "NO");
        }

        #[test]
        fn test3() {
            let data = b"2\nUU\n";
            let reader = BufReader::new(&data[..]);
            let result = game(reader.lines());
            assert_eq!(result, "NO");
        }
}
