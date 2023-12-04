use regex::Regex;
use adventOfCode2023::read_from_file;

fn main() {
    let values = read_from_file("src/day2/input.txt");

    let limit = Set {
        green: Cube::Green(13),
        red: Cube::Red(12),
        blue: Cube::Blue(14),
    };

    let games = Games::play(values, limit);

    println!("{:?}", games.sum_of_possible_games_ids());
}


struct Games {
    games: Vec<Game>,
    limit: Set,
}

impl Games {
    pub fn play(values: Vec<String>, limit: Set) -> Games {
        let games: Vec<Game> = values
            .iter()
            .map(|game| { Game::new(game) })
            .collect();
        Games {
            games,
            limit,
        }
    }

    pub fn sum_of_possible_games_ids(self) -> usize {
        let v = self.games.iter().filter(|v| {
            v.sets.iter().filter(|c| {
                !c.is_valid(&self.limit)
            }).count() == 0
        })
            .map(|g| g.id)
            .sum();
        v
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
    pub fn new(red: Cube, blue: Cube, green: Cube) -> Self {
        Set { red, blue, green }
    }
    pub fn is_valid(&self, limit: &Set) -> bool {
        if self.red <= limit.red && self.blue <= limit.blue && self.green <= limit.green {
            return true;
        }
        return false;
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    pub fn new(game: &str) -> Self {
        let id_regex = Regex::new(r"Game (?P<game>\d+):").unwrap();
        let id = id_regex
            .captures(game)
            .unwrap()
            .name("game")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let sets: Vec<Set> = game
            .split(';')
            .map(|s| {
                let red = Game::extract_value(s, "red");
                let blue = Game::extract_value(s, "blue");
                let green = Game::extract_value(s, "green");
                Set::new(
                    Cube::Red(red),
                    Cube::Blue(blue),
                    Cube::Green(green),
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
    use crate::{Cube, Games, Set};

    #[test]
    fn should_get_sum_of_all_possible_games_ids() {
        let limit = Set {
            green: Cube::Green(13),
            red: Cube::Red(12),
            blue: Cube::Blue(14),
        };

        let games = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
        let games = Games::play(games, limit);

        let ids: usize = games.sum_of_possible_games_ids();

        assert_eq!(ids, 8);
    }
}
