use regex::Regex;

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
    let str = include_str!("input.txt");
    let sum = get_calibration_values(str);
    println!("{:?}", sum);
    println!("{:?}", sum.iter().sum::<u32>());
}

const REGEX: &str = r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|([0-9])";

fn string_to_number(string: &str) -> u32 {
    match string {
        "zero" => 0,
        "0" => 0,
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => unreachable!("Should have everything covered!")
    }
}

fn get_calibration_values(input: &str) -> Vec<u32> {
    let re = Regex::new(REGEX).unwrap();

    for line in input.lines() {
        let num = match re.find_iter(line)
            .map(|m| string_to_number(m.as_str()))
            .collect::<Vec<_>>()
            .as_slice() {
                [x] => x * 10 + x,
                [x, .., y] => x * 10 + y,
                _ => unreachable!("The vec should not be able to be empty!")
            };
        println!("String is: {} and number is {}", line, num);
    }

    vec![]

    /* input.lines()
        .map(|line| {
            match re.find_iter(line)
                .map(|m| string_to_number(m.as_str()))
                .collect::<Vec<_>>()
                .as_slice() {
                    [x] => x * 10 + x,
                    [x, .., y] => x * 10 + y,
                    _ => unreachable!("The vec should not be able to be empty!")
                }})
        .collect() */

    /* for line in input.lines() {
        match re.find_iter(line)
            .map(|m| string_to_number(m.as_str()))
            .collect::<Vec<_>>()
            .as_slice() {
                [x] => {
                vec![x.to_owned(), x.to_owned()].iter().fold(0, |a, i| a * 10 + i)
                },
                [x, .., y] => {
                    vec![x.to_owned(), y.to_owned()].iter().fold(0, |a, i| a * 10 + i)
                },
                _ => unreachable!("The vec should not be able to be empty!")
            };
    } */
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
        
        assert_eq!(get_calibration_values(str), vec![29,83,13,24,42,14,76]);
    }
}