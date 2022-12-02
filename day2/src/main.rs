use std::str::FromStr;
use std::{fs, io};

enum Choice {
    Rock,     // 1
    Paper,    // 2
    Scissors, // 3
}

// loss 0
// draw 3
// win 6

struct Game {
    opp: Choice,
    you: Choice,
}

impl Game {
    fn score(&self) -> i32 {
        match (&self.opp, &self.you) {
            (Choice::Rock, Choice::Scissors) => 3,
            (Choice::Rock, Choice::Rock) => 1 + 3,
            (Choice::Rock, Choice::Paper) => 2 + 6,
            (Choice::Paper, Choice::Rock) => 1,
            (Choice::Paper, Choice::Paper) => 2 + 3,
            (Choice::Paper, Choice::Scissors) => 3 + 6,
            (Choice::Scissors, Choice::Paper) => 2,
            (Choice::Scissors, Choice::Scissors) => 3 + 3,
            (Choice::Scissors, Choice::Rock) => 1 + 6,
        }
    }
}

impl FromStr for Game {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let choices = input.split_whitespace().collect::<Vec<&str>>();
        let opp = match_choice(choices[0]);
        let you = match_choice(choices[1]);

        Ok(Game { opp, you })
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let batches = input.split("\n").collect::<Vec<&str>>();

    let games = batches.iter().fold(vec![], |mut acc, b| {
        if !b.is_empty() {
            acc.push(b.parse::<Game>().unwrap());
            acc
        } else {
            acc
        }
    });

    let score = games.iter().map(|g| g.score()).sum::<i32>();
    println!("{:?}", score);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let batches = input.split("\n").collect::<Vec<&str>>();

    let mut games: Vec<Game> = vec![];

    for batch in batches {
        let choices = batch.split_whitespace().collect::<Vec<&str>>();
        if !choices.is_empty() {
            let opp = match_choice(choices[0]);
            let you = match_choice(choices[1]);

            let you = match you {
                Choice::Rock => match opp {
                    Choice::Rock => Choice::Scissors,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissors => Choice::Paper,
                },
                Choice::Paper => match opp {
                    Choice::Rock => Choice::Rock,
                    Choice::Paper => Choice::Paper,
                    Choice::Scissors => Choice::Scissors,
                },
                Choice::Scissors => match opp {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissors,
                    Choice::Scissors => Choice::Rock,
                },
            };

            let game = Game { opp, you };

            games.push(game);
        }
    }

    let score = games.iter().map(|g| g.score()).sum::<i32>();
    println!("{:?}", score);
}

fn match_choice(c: &str) -> Choice {
    match c {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => unreachable!(),
    }
}
