fn parse_pairs(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|segment| {
            let mut parts = segment.trim().split('-');
            let start = parts.next()?.trim().parse().ok()?;
            let end = parts.next()?.trim().parse().ok()?;
            Some((start, end))
        })
        .collect()
}

fn is_invalid(n: u64) -> bool {
    let nb_digits = get_number_of_digits(n);
    if nb_digits % 2 == 0 {
        let left = n / 10_u64.pow((nb_digits / 2) as u32);
        let right = n % 10_u64.pow((nb_digits / 2) as u32);
        log::debug!("n = {n}, left = {left}, right = {right}");
        left == right
    } else {
        false
    }
}

fn get_number_of_digits(n: u64) -> u32 {
    let power = (n as f64).log10().floor() as u32;
    let nb_digits = power + 1;
    nb_digits
}

fn is_invalid_2(n: u64) -> bool {
    let nb_digits = get_number_of_digits(n);
    for split in 2..=nb_digits {
        if nb_digits % split == 0 {
            let size = nb_digits / split;
            let right = n % 10_u64.pow(size as u32);
            let mut zombie = right;
            for segment in 1..split {
                zombie += right * 10_u64.pow(size * segment as u32);
            }
            log::debug!("n = {n}, split = {split}, right = {right}, zombie = {zombie}");
            if n == zombie {
                return true
            }
        }
    }
    false
}

fn process<F>(pairs: Vec<(u64, u64)>, predicate: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    let mut invalid_numbers = Vec::new();
    for pair in pairs {
        for n in pair.0..=pair.1 {
            if predicate(n) {
                println!("Invalid id {n}");
                invalid_numbers.push(n);
            }
        }
    }
    invalid_numbers
}

fn main() {
    env_logger::init();
    let mode = std::env::args().nth(1).expect("expected argument");
    let pairs = std::env::args().nth(2).expect("expected argument");
    let pairs = parse_pairs(&pairs);
    log::debug!("{pairs:?}");
    let predicate = match mode.as_str() {
        "1" => is_invalid,
        "2" => is_invalid_2,
        _ => panic!("Unexpected")
    };
    let invalid_numbers =process(pairs, predicate);
    log::debug!("Invalid numbers: {:?}", invalid_numbers);
    let sum: u64 = invalid_numbers.iter().sum();
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_2_basic() {
        // 123123: split=2, size=3, left=123, right=123
        assert!(is_invalid_2(123123));
        // 121212: split=3, size=2, parts=[12,12,12]
        assert!(is_invalid_2(121212));
        // 1111: split=2, size=2, left=11, right=11
        assert!(is_invalid_2(1111));
    }

    #[test]
    fn test_is_invalid_2_edge_cases() {
        // Single digit
        assert!(!is_invalid_2(1));
        // Two digits, same
        assert!(is_invalid_2(11));
        // Large number with repeated pattern
        assert!(is_invalid_2(123412341234));
    }

    #[test]
    fn test_is_invalid_2_false() {
        // Single digit
        assert!(!is_invalid_2(1));
        // Two digits, same
        assert!(!is_invalid_2(123));
    }
}
