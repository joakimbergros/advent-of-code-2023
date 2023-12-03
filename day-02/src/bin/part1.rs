/// --- Day 2: Cube Conundrum ---
/// 
/// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
/// 
/// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
/// 
/// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
/// 
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
/// 
/// You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
/// 
/// For example, the record of a few games might look like this:
/// 
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// 
/// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
/// 
/// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
/// 
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.
/// 
/// Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let count = input.lines()
        .map(|line| {
            let mut game = Game::default();
            for (index, value) in line
                .split(|s|  s == ':' || s == ';' || s == ',')
                .enumerate() {
                if index == 0 {
                    //game_number = value[5..].parse::<u32>().unwrap();
                    game.number = value[5..].parse::<u32>().unwrap();
                } else {
                    let count = value.trim().split(' ').next().unwrap().parse::<u32>().unwrap();
                    let mut color = Color::Undefined;
                    if value.ends_with("red") {
                        color = Color::Red;
                    } else if value.ends_with("green") {
                        color = Color::Green;
                    } else if value.ends_with("blue") {
                        color = Color::Blue;
                    }
                    game.set_count(count, color);
                }
            }
            game
        })
        .fold(0, |acc, game| {
            if !game.overflown {
                acc + game.number
            } else {
                acc
            }
        });
    println!("{count}");
    count.to_string()
}

#[derive(Default, Debug)]
struct Game {
    number: u32,
    red: u32,
    green: u32,
    blue: u32,
    overflown: bool
}

impl Game {
    fn set_count(&mut self, count: u32, color: Color) {
        match color {
            Color::Red => {
                if count > 12 {
                    self.overflown = true;
                };

                if count < self.red {
                    return;
                }

                self.red = count;
            },
            Color::Green => {
                if count > 13 {
                    self.overflown = true;
                };

                if count < self.green {
                    return;
                }
                self.green = count;
            },
            Color::Blue => {
                if count > 14 {
                    self.overflown = true;
                };

                if count < self.blue {
                    return;
                }
                self.blue = count;
            },
            Color::Undefined => panic!("Should not be used")
        }
    }
}   

enum Color {
    Undefined,
    Red,
    Green,
    Blue
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
        
        assert_eq!("8", process_input(str));
    }
}