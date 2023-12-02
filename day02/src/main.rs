use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl Game {
    pub fn update(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn is_valid(&self, red: u64, green: u64, blue: u64) -> bool {
        self.rounds.iter().all(|x| x.is_valid(red, green, blue))
    }
}

#[derive(Debug)]
pub struct Round {
    red: u64,
    blue: u64,
    green: u64,
}

impl Round {
    pub fn is_valid(&self, red: u64, green: u64, blue: u64) -> bool {
        self.red <= red && self.blue <= blue && self.green <= green
    }
}

fn main() {
    let content = read_file("./input.txt").unwrap();

    let mut games = vec![];
    for line in content.lines() {
        games.push(process_string(line));
    }

    let res = games
        .iter()
        .filter(|x| x.is_valid(12, 13, 14))
        .map(|game| game.id)
        .sum::<u64>();
    println!("Part 01 Result: {}", res);

    let res: u64 = games
        .iter()
        .map(|game| {
            let min_red = game.rounds.iter().map(|round| round.red).max().unwrap();
            let min_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();
            let min_green = game.rounds.iter().map(|round| round.green).max().unwrap();

            min_red * min_blue * min_green
        })
        .sum();
    println!("Part 02 Result: {}", res);
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn process_string(s: &str) -> Game {
    let parts = s.split(':').collect::<Vec<&str>>();
    let id = parts[0]
        .strip_prefix("Game ")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let mut game = Game {
        id: id as u64,
        rounds: vec![],
    };

    let balls = parts[1].split(';').collect::<Vec<&str>>();
    for ball in balls {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        let colors = ball.split(',').collect::<Vec<&str>>();
        for ball in colors {
            let parts = ball
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            let num = parts[0].parse::<u64>().unwrap();
            let color = parts[1];

            match color {
                "red" => round.red = num,
                "green" => round.green = num,
                "blue" => round.blue = num,
                _ => (),
            }
        }
        game.update(round);
    }
    game
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_01() {
        let content = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let games = content
            .iter()
            .map(|x| process_string(x))
            .collect::<Vec<Game>>();

        let res = games
            .iter()
            .filter(|x| x.is_valid(12, 13, 14))
            .map(|game| game.id)
            .sum::<u64>();

        assert_eq!(res, 8);
    }

    #[test]
    fn part_02() {
        let content = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let games = content
            .iter()
            .map(|x| process_string(x))
            .collect::<Vec<Game>>();

        let res = games
            .iter()
            .map(|game| {
                let min_red = game.rounds.iter().map(|round| round.red).max().unwrap();
                let min_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();
                let min_green = game.rounds.iter().map(|round| round.green).max().unwrap();

                min_red * min_blue * min_green
            })
            .sum::<u64>();

        assert_eq!(res, 2286);
    }
}
