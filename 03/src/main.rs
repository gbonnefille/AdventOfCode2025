use std::io::{self, BufRead};

fn extract_batteries(banck: &str) -> Vec<u32> {
    banck.chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

fn get_power(bank: Vec<u32>) -> u32 {
    if bank.len() < 2 {
        return 0;
    }
    let (max1_idx, max1_val) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .fold((0, 0), |acc, (i, &v)| {
            if v > acc.1 {
                (i, v)
            } else {
                acc
            }
        });

    let max2_val = bank[max1_idx + 1..]
        .iter()
        .max()
        .copied()
        .unwrap_or(0);

    max1_val * 10 + max2_val
}

fn main() {
    let stdin = io::stdin();
    let sum: u32 = stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|line| {
            let bank = extract_batteries(&line);
            get_power(bank)
        })
        .sum();

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_power_with_3_digits() {
        let bank = extract_batteries("123");
        assert_eq!(get_power(bank), 23);

        let bank = extract_batteries("391");
        assert_eq!(get_power(bank), 91);

        let bank = extract_batteries("789");
        assert_eq!(get_power(bank), 89);

        let bank = extract_batteries("321");
        assert_eq!(get_power(bank), 32);
    }

    #[test]
    fn test_get_power_with_4_digits() {
        let bank = extract_batteries("4567");
        assert_eq!(get_power(bank), 67);

        let bank = extract_batteries("9081");
        assert_eq!(get_power(bank), 98);

        let bank = extract_batteries("1002");
        assert_eq!(get_power(bank), 12);

        let bank = extract_batteries("9998");
        assert_eq!(get_power(bank), 99);
    }
}
