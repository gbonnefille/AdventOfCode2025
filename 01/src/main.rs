use std::io::{self, BufRead};
use std::io::Cursor;

fn read_instructions_from<R: BufRead>(reader: R) -> Vec<(char, i32)> {
    let mut instructions = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let l = l.trim();
            if l.len() >= 2 {
                let dir = l.chars().next().unwrap();
                let num = l[1..].trim().parse::<i32>().unwrap_or(0);
                if dir == 'L' || dir == 'R' {
                    instructions.push((dir, num));
                }
            }
        }
    }
    instructions
}

fn read_instructions() -> Vec<(char, i32)> {
    let stdin = io::stdin();
    read_instructions_from(stdin.lock())
}

fn main() {
    let instructions = read_instructions();
    for (dir, num) in instructions {
        println!("Direction: {}, Number: {}", dir, num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_instructions_sample_input() {

        let input = b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let cursor = Cursor::new(&input[..]);
        let instructions = read_instructions_from(cursor);

        let expected = vec![
            ('L', 68),
            ('L', 30),
            ('R', 48),
            ('L', 5),
            ('R', 60),
            ('L', 55),
            ('L', 1),
            ('L', 99),
            ('R', 14),
            ('L', 82),
        ];
        assert_eq!(instructions, expected);
    }
}
