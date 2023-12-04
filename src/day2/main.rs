use regex::Regex;
use std::fs;

fn main() {
    let file_path = "src/day2/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let values: Vec<_> = contents.split('\n').collect();

    let games = Games::play(values);

    println!("{:?}", games.sum_of_possible_games_ids());
}


struct Games {
    games: Vec<Game>,
}

impl Games {
    pub fn play(values: Vec<&str>) -> Games {
        let games: Vec<Game> = values
            .iter()
            .map(|game| {
                Game::new(
                    game,
                    &Set {
                        green: Cube::Green(13),
                        red: Cube::Red(12),
                        blue: Cube::Blue(14),
                    },
                )
            })
            .filter(|g| g.is_possible())
            .collect();
        Games {
            games,
        }
    }
    pub fn sum_of_possible_games_ids(&self) -> usize {
        let ids: usize = self.games.iter().map(|g| g.id).sum();
        ids
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

#[derive(Debug, Clone)]
struct Set {
    red: Cube,
    blue: Cube,
    green: Cube,
}

impl Set {
    pub fn new(red: Cube, blue: Cube, green: Cube, limit: &Set) -> Option<Self> {
        if red <= limit.red && blue <= limit.blue && green <= limit.green {
            Some(Set { red, blue, green })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Option<Set>>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        let v: Vec<_> = self.sets.iter().filter(|v| v.is_none()).collect();
        v.is_empty()
    }
}

impl Game {
    pub fn new(game: &str, bag_limit: &Set) -> Self {
        let id_regex = Regex::new(r"Game (?P<game>\d+):").unwrap();
        let id = id_regex
            .captures(game)
            .unwrap()
            .name("game")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let sets: Vec<Option<Set>> = game
            .split(';')
            .map(|s| {
                let red = Game::extract_value(s, "red");
                let blue = Game::extract_value(s, "blue");
                let green = Game::extract_value(s, "green");
                Set::new(
                    Cube::Red(red),
                    Cube::Blue(blue),
                    Cube::Green(green),
                    bag_limit,
                )
            })
            .collect();

        Game { id, sets }
    }

    fn extract_value(input: &str, capture: &str) -> usize {
        let reg = format!(r"(?P<group>\d+) {capture}");
        let red_regex = Regex::new(reg.as_str()).unwrap();

        match red_regex.captures(input) {
            None => 0,
            Some(c) => c.name("group").unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Games;

    #[test]
    fn test() {
        let games = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games = Games::play(games);

        let ids: usize = games.sum_of_possible_games_ids();

        assert_eq!(ids, 8);
    }
}
