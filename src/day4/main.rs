use regex::Regex;
use adventOfCode2023::read_from_file;

fn main() {
    let values = read_from_file("src/day4/input.txt");
    let games: Vec<Game> = values.iter().map(|game| Game::new(game)).collect();
    let total = Score{
        games
    }.total_score();
    println!("{:?}", total);
}

struct Score {
    games: Vec<Game>,
}

impl Score {
    pub fn total_score(&self) -> usize {
        self.games.iter().map(|g| g.points()).sum()
    }
}

#[derive(Debug)]
struct Game {
    winning: Vec<usize>,
    mines: Vec<usize>,
}

impl Game {
    pub fn new(game: &str) -> Self {
        let id_regex = Regex::new(r"Card .*: (?P<winners>[\d\s]+) \| (?P<mine>[\d\s]+)").unwrap();

        let mines = Self::extract(game, &id_regex, "mine");

        let winning = Self::extract(game, &id_regex, "winners");


        Game {
            winning,
            mines,
        }
    }
    fn points(&self) -> usize {
        let matches = self.winning
            .iter()
            .filter(|v| self.mines.contains(v))
            .count();
        match matches {
            0 => 0,
            i => (1..=i).reduce(|acc, e| acc*2).unwrap()
        }
    }

    fn extract(game: &str, id_regex: &Regex, x: &str) -> Vec<usize> {
        let values: Vec<usize> = id_regex
            .captures(game)
            .unwrap()
            .name(x)
            .unwrap()
            .as_str()
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        values
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::{Game, Score};

    #[test]
    fn should_get_points_from_game() {
        let value = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let game = Game::new(value);
        println!("{:?}", game);

        assert_eq!(game.points(), 8);
    }

    #[test]
    fn should_get_points_from_various_games() {
        let values = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let games: Vec<Game> = values.iter().map(|v| Game::new(v)).collect();
        let points = Score {
            games
        }.total_score();


        assert_eq!(points, 13);
    }


}

