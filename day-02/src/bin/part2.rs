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
            acc + game.red * game.green * game.blue
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
        
        assert_eq!("2286", process_input(str));
    }
}