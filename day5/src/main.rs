use std::fs;
use std::str;
use std::str::FromStr;
use std::vec;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .split_ascii_whitespace()
            .filter(|s| s.parse::<usize>().is_ok())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Ok(Instruction {
            amount: *instructions.get(0).unwrap(),
            from: *instructions.get(1).unwrap(),
            to: *instructions.get(2).unwrap(),
        })
    }
}

fn parse_letter(s: &str) -> char {
    s.chars().find(|&c| c != '[' && c != ']').unwrap().into()
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (crates_lines, instructions_lines) = input.split_once("\n\n").unwrap();

    let mut stack_data = crates_lines
        .lines()
        .map(|l| {
            l.as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .map(|s| s.unwrap().trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let stack_nos = stack_data.pop().unwrap();

    stack_data.reverse();

    let mut stacks = stack_data
        .iter()
        .fold(vec![vec![]; stack_nos.len()], |mut acc, row| {
            for (i, c) in row.iter().enumerate() {
                if !c.is_empty() {
                    acc[i].push(parse_letter(c));
                }
            }
            acc
        });

    let instructions = instructions_lines
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for instruction in instructions {
        for n in 0..instruction.amount {
            let pop_crate = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(pop_crate);
        }
    }

    let output = stacks.iter().fold(String::new(), |mut acc, stack| {
        if stack.last().is_some() {
            acc.push(*stack.last().unwrap());
        }
        acc
    });

    println!("{:?}", output);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (crates_lines, instructions_lines) = input.split_once("\n\n").unwrap();

    let mut stack_data = crates_lines
        .lines()
        .map(|l| {
            l.as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .map(|s| s.unwrap().trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let stack_nos = stack_data.pop().unwrap();

    stack_data.reverse();

    let mut stacks = stack_data
        .iter()
        .fold(vec![vec![]; stack_nos.len()], |mut acc, row| {
            for (i, c) in row.iter().enumerate() {
                if !c.is_empty() {
                    acc[i].push(parse_letter(c));
                }
            }
            acc
        });

    let instructions = instructions_lines
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for instruction in instructions {
        let mut crates_to_move = vec![];
        for _n in 0..instruction.amount {
            crates_to_move.push(stacks[instruction.from - 1].pop().unwrap());
        }

        crates_to_move.reverse();

        for c in crates_to_move {
            stacks[instruction.to - 1].push(c);
        }
    }

    let output = stacks.iter().fold(String::new(), |mut acc, stack| {
        if stack.last().is_some() {
            acc.push(*stack.last().unwrap());
        }
        acc
    });

    println!("{:?}", output);
}
