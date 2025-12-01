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

fn apply_instructions(start: i32, instructions: &[(char, i32)]) -> i32 {
    let mut value = start;
    for &(dir, num) in instructions {
        match dir {
            'L' => value = value - num, // Inversé : 'L' soustrait
            'R' => value = value + num, // Inversé : 'R' ajoute
            _ => {}
        }
        value = value.rem_euclid(100);
        println!("Current value: {}", value)
    }
    value.rem_euclid(100)
}

fn main() {
    let instructions = read_instructions();
    for (dir, num) in &instructions {
        println!("Direction: {}, Number: {}", dir, num);
    }
    apply_instructions(50, &instructions);
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

    #[test]
    fn test_apply_single_instruction_parametrized() {
        let cases = vec![
            (0, ('L', 10), 90),      // 0 - 10 = -10 rem_euclid(100) = 90
            (10, ('R', 5), 15),      // 10 + 5 = 15
            (98, ('L', 2), 96),      // 98 - 2 = 96
            (1, ('R', 3), 4),        // 1 + 3 = 4
            (0, ('R', 1), 1),        // 0 + 1 = 1
            (99, ('L', 1), 98),      // 99 - 1 = 98
            (99, ('R', 1), 0),       // 99 + 1 = 100 rem_euclid(100) = 0
            (0, ('L', 99), 1),       // 0 - 99 = -99 rem_euclid(100) = 1
            (0, ('R', 99), 99),      // 0 + 99 = 99
        ];
        for (start, instruction, expected) in cases {
            let result = apply_instructions(start, &[instruction]);
            assert_eq!(result, expected, "Failed for start={}, instruction={:?}", start, instruction);
        }
    }

    #[test]
    fn test_apply_instructions_from_zero() {
        let instructions = vec![
            ('L', 10), // 0 - 10 = -10 rem_euclid(100) = 90
            ('R', 5),  // 90 + 5 = 95
            ('L', 94), // 95 - 94 = 1
            ('R', 1),  // 1 + 1 = 2
        ];
        let result = apply_instructions(0, &instructions);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_apply_instructions_from_99() {
        let instructions = vec![
            ('R', 1),  // 99 + 1 = 100 rem_euclid(100) = 0
            ('L', 2),  // 0 - 2 = -2 rem_euclid(100) = 98
            ('R', 3),  // 98 + 3 = 101 rem_euclid(100) = 1
        ];
        let result = apply_instructions(99, &instructions);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_read_instructions_invalid_lines() {
        let input = b"L10\nX20\nR5\nLabc\nR-3\n\nL 7\n";
        let cursor = Cursor::new(&input[..]);
        let instructions = read_instructions_from(cursor);

        let expected = vec![
            ('L', 10),
            ('R', 5),
            ('L', 0),
            ('R', -3),
            ('L', 7),
        ];
        assert_eq!(instructions, expected);
    }

    #[test]
    fn test_apply_instructions_empty() {
        let instructions: Vec<(char, i32)> = vec![];
        let result = apply_instructions(42, &instructions);
        assert_eq!(result, 42);
    }
}
