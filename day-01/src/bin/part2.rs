use aho_corasick::AhoCorasick;

fn text_to_num(text: &str) -> Option<u32> {
    match text {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_line(line: &str) -> Option<u32> {
    let mut first_digit : Option<u32> = None;
    let mut last_digit : Option<u32> = None;

    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let ac = AhoCorasick::new(patterns).unwrap();

    fn do_matches(matched_str: &str, first_digit : &mut Option<u32>, last_digit : &mut Option<u32>) {
        if matched_str.len() > 1 {
            if let Some(num) = text_to_num(matched_str) {
                if first_digit.is_none() {
                    *first_digit = Some(num);
                }
                *last_digit = Some(num);
            }
        } else {
            if let Some(digit) = matched_str.parse::<u32>().ok() {
                // ok converts a Result to an Option
                if first_digit.is_none() {
                    *first_digit = Some(digit);
                }
                *last_digit = Some(digit);
            }
        }
    }
    
    for matched in ac.find_overlapping_iter(line) {
        // Could not use a regex with the regex crate because it does not support overlapping matches
        do_matches(patterns[matched.pattern()], &mut first_digit, &mut last_digit);
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        Some(first * 10 + last)
    } else {
        None // return None if no digits were found
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let answer = input
        .lines()
        .flat_map(|line| parse_line(line))
        .sum::<u32>();

    // Print the answer
    println!("{:?}", answer);
}