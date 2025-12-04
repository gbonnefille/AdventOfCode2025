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
    let power = (n as f64).log10().floor() as u32;
    let nb_digits = power + 1;
    if nb_digits % 2 == 0 {
        let left = n / 10_u64.pow((nb_digits / 2) as u32);
        let right = n % 10_u64.pow((nb_digits / 2) as u32);
        log::debug!("n = {n}, left = {left}, right = {right}");
        left == right
    } else {
        false
    }
}

fn main() {
    let arg = std::env::args().nth(1).expect("expected argument");
    let pairs = parse_pairs(&arg);
    env_logger::init();
    log::debug!("{pairs:?}");
    let mut invalid_numbers = Vec::new();
    for pair in pairs {
        for n in pair.0..=pair.1 {
            if is_invalid(n) {
                println!("Invalid id {n}");
                invalid_numbers.push(n);
            }
        }
    }
    log::debug!("Invalid numbers: {:?}", invalid_numbers);
    let sum: u64 = invalid_numbers.iter().sum();
    println!("Sum: {sum}");
}
