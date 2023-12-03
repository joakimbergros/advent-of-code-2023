// --- Part Two ---
// 
// The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!
// 
// As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
// 
// Again consider the example games from earlier:
// 
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// 
//     In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
//     Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
//     Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
//     Game 4 required at least 14 red, 3 green, and 15 blue cubes.
//     Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
// 
// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.
// 
// For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
// 
fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let test = input.lines()
        .map(|line| {
            let (mut game_number, mut red_count, mut green_count, mut blue_count, mut possible) = (0 as u32, 0 as u32, 0 as u32, 0 as u32, true);
            for (index, value) in line
                .split(|s|  s == ':' || s == ';' || s == ',')
                .enumerate() {
                if index == 0 {
                    game_number = value[5..].parse::<u32>().unwrap();
                } else {
                    let count = value.trim().split(' ').next().unwrap().parse::<u32>().unwrap();
                    if value.ends_with("red") && count > red_count {
                        red_count = count;
                        if red_count > 12 {
                            possible = false
                        }
                    } else if value.ends_with("green") && count > green_count {
                        green_count = count;
                        if green_count > 13 {
                            possible = false
                        }
                    } else if value.ends_with("blue") && count > blue_count {
                        blue_count = count;
                        if blue_count > 14 {
                            possible = false
                        }
                    }
                }
            }

            println!("{:?} - {:?} - {:?} - {:?} - {:?}", game_number, red_count, green_count, blue_count, possible);
            (game_number, red_count, green_count, blue_count, possible)
        })
        .fold(0, |acc, val| {
            acc + val.1 * val.2 * val.3
        });
    println!("{test}");
    test.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        
        assert_eq!("2286", process_input(str));
    }
}