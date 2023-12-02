/// --- Part Two ---
/// 
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
/// 
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
/// 
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// 
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
/// 
/// What is the sum of all of the calibration values?
fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let output = input
        .lines()
        .map(process_line)
        .sum::<u32>();

    output.to_string()
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let better_chars = std::iter::from_fn(move || {
        let reduced_line = &line[index..];
        index += 1;

        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            reduced_line.chars().next()
        };

        result
    });

    let mut it = better_chars
        .filter_map(|character| {
            character.to_digit(10)
        });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        
        assert_eq!("281", process_input(str));
    }
}