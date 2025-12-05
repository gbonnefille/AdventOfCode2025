use std::io::{self, BufRead};

fn extract_batteries(banck: &str) -> Vec<u32> {
    banck.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn get_power(bank: Vec<u32>, nb: usize) -> u64 {
    if bank.len() < 2 {
        return 0;
    }
    let mut max_vals = Vec::new();

    let mut cur_idx = 0;
    for n in 1..nb {
        let end = bank.len() - (nb - n);
        let (max_idx, max_val) = bank[cur_idx..end]
            .iter()
            .enumerate()
            .fold((0, 0), |acc, (i, &v)| if v > acc.1 { (i, v) } else { acc });
        
        log::debug!("{cur_idx}..{end} -> {max_idx}");

        max_vals.push(max_val);
        cur_idx = max_idx + cur_idx + 1;
    }

    let max2_val = bank[cur_idx..].iter().max().copied().unwrap_or(0);

    max_vals.push(max2_val);

    let sum: u64 = max_vals
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &val)| val as u64 * 10u64.pow(i as u32))
        .sum();

        log::debug!("{bank:?} -> {sum}");
    sum
}

fn full_power(data: impl Iterator<Item = String>, nb: usize) -> u64 {
    data.map(|line| {
        let bank = extract_batteries(&line);
        get_power(bank, nb)
    })
    .sum()
}

fn main() {
    let nb = std::env::args()
        .nth(1)
        .expect("Missing argument: nb")
        .parse::<usize>()
        .expect("Argument nb must be a positive integer");
    let stdin = io::stdin();
    let data = stdin.lock().lines().filter_map(Result::ok);
    let sum = full_power(data,nb);

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_power_part1() {
        let input = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|s| s.to_string());
        let sum = full_power(input, 2);
        assert_eq!(357, sum);
    }

    #[test]
    fn test_full_power_part2() {
        let input = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|s| s.to_string());
        let sum = full_power(input,12);
        assert_eq!(3121910778619, sum);
    }

    #[test]
    fn test_get_power_with_3_digits() {
        let bank = extract_batteries("123");
        assert_eq!(get_power(bank, 2), 23);

        let bank = extract_batteries("391");
        assert_eq!(get_power(bank, 2), 91);

        let bank = extract_batteries("789");
        assert_eq!(get_power(bank, 2), 89);

        let bank = extract_batteries("321");
        assert_eq!(get_power(bank, 2), 32);
    }

    #[test]
    fn test_get_power_with_4_digits() {
        let bank = extract_batteries("4567");
        assert_eq!(get_power(bank, 2), 67);

        let bank = extract_batteries("9081");
        assert_eq!(get_power(bank, 2), 98);

        let bank = extract_batteries("1002");
        assert_eq!(get_power(bank, 2), 12);

        let bank = extract_batteries("9998");
        assert_eq!(get_power(bank, 2), 99);
    }
}
