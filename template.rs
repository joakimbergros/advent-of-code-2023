use nom::IResult;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process(str);
    println!("{sum}");
}

fn parse(input: &str) -> IResult<&str, ()> {
    ()
}

fn process(input: &str) -> String {
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "";

        assert_eq!("", process(str));
    }
}
