fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!("", process_input(str));
    }
}
