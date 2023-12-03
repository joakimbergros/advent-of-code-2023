/// 
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
        let str = "";
        
        assert_eq!("", process_input(str));
    }
}