use advent_of_code_2025_day_2::{parse_pairs, is_invalid, is_invalid_2, process};

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
