fn extract_number(line: &str) -> Option<u32> {
    let mut first_digit : Option<u32> = None;
    let mut last_digit : Option<u32> = None;

    for c in line.chars() {
       if let Some(digit) = c.to_digit(10) {
           if first_digit.is_none() {
               first_digit = Some(digit);
           }
           last_digit = Some(digit);
       }
    }

    // Combine the first and last digits into a two-digit number
    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        Some(first * 10 + last)
    } else {
        None // return None if no digits were found
    }
}

fn main() {
    // Read from input.txt
    let input = include_str!("./input.txt");
    // unwrap deals with all the Result thingies and panics if there's an error (instead of Ok), so you can just use the underlying value
    let answer : u32 = input.lines()
        .flat_map(|line| extract_number(line)).sum();
    println!("Answer: {}", answer);
}