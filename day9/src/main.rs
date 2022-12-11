use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq)]
enum Direction {
    Top,
    TopRight,
    TopLeft,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Overlap,
}

#[derive(Debug)]
enum InstructionDirection {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Instruction {
    direction: InstructionDirection,
    steps: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_at(1);
        let steps = steps.trim().parse::<i32>().unwrap();
        let direction = match direction {
            "U" => InstructionDirection::Up,
            "L" => InstructionDirection::Left,
            "R" => InstructionDirection::Right,
            "D" => InstructionDirection::Down,
            _ => panic!("Unknown direction"),
        };

        Ok(Instruction { direction, steps })
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn print_table(&self) {
        let mut table = vec![vec![' '; 50]; 50];
        for knot in self.knots.iter() {
            table[(knot.1 + 25) as usize][(knot.0 + 25) as usize] = 'X';
        }
        for row in table.iter().rev() {
            for column in row.iter() {
                print!("{}", column);
            }
            println!();
        }
    }

    fn move_direction(&mut self, instruction: &Instruction) {
        let mut ahead_direction = Direction::Overlap;

        let knots = self.knots.clone();
        let mut move_direction = match instruction.direction {
            InstructionDirection::Up => Direction::Top,
            InstructionDirection::Left => Direction::Left,
            InstructionDirection::Right => Direction::Right,
            InstructionDirection::Down => Direction::Bottom,
        };

        for (index, knot) in self.knots.iter_mut().enumerate() {
            if index == 0 {
                ahead_direction = get_direction(*knot, knots[index + 1]);
                match instruction.direction {
                    InstructionDirection::Up => knot.1 += 1,
                    InstructionDirection::Left => knot.0 -= 1,
                    InstructionDirection::Right => knot.0 += 1,
                    InstructionDirection::Down => knot.1 -= 1,
                }

                continue;
            }

            let mut current_knot = knot.clone();

            match move_direction {
                Direction::Top => match ahead_direction {
                    Direction::Top => {
                        move_coordinate(&mut current_knot, Direction::Top);
                    }

                    Direction::TopRight => {
                        move_coordinate(&mut current_knot, Direction::TopRight);
                    }

                    Direction::TopLeft => {
                        move_coordinate(&mut current_knot, Direction::TopLeft);
                    }

                    _ => {}
                },
                Direction::TopRight => match ahead_direction {
                    Direction::Top => {
                        move_coordinate(&mut current_knot, Direction::TopRight);
                    }

                    Direction::TopRight => {
                        move_coordinate(&mut current_knot, Direction::TopRight);
                    }

                    Direction::TopLeft => {
                        move_coordinate(&mut current_knot, Direction::Top);
                    }

                    Direction::BottomRight => {
                        move_coordinate(&mut current_knot, Direction::Right);
                    }

                    Direction::Right => {
                        move_coordinate(&mut current_knot, Direction::TopRight);
                    }

                    _ => {}
                },
                Direction::TopLeft => match ahead_direction {
                    Direction::Top => {
                        move_coordinate(&mut current_knot, Direction::TopLeft);
                    }

                    Direction::TopLeft => {
                        move_coordinate(&mut current_knot, Direction::TopLeft);
                    }

                    Direction::TopRight => {
                        move_coordinate(&mut current_knot, Direction::Top);
                    }

                    Direction::BottomLeft => {
                        move_coordinate(&mut current_knot, Direction::Left);
                    }

                    Direction::Left => {
                        move_coordinate(&mut current_knot, Direction::TopLeft);
                    }

                    _ => {}
                },
                Direction::Right => match ahead_direction {
                    Direction::TopRight => {
                        move_coordinate(&mut current_knot, Direction::TopRight);
                    }

                    Direction::Right => {
                        move_coordinate(&mut current_knot, Direction::Right);
                    }

                    Direction::BottomRight => {
                        move_coordinate(&mut current_knot, Direction::BottomRight);
                    }

                    _ => {}
                },
                Direction::Left => match ahead_direction {
                    Direction::TopLeft => {
                        move_coordinate(&mut current_knot, Direction::TopLeft);
                    }

                    Direction::Left => {
                        move_coordinate(&mut current_knot, Direction::Left);
                    }

                    Direction::BottomLeft => {
                        move_coordinate(&mut current_knot, Direction::BottomLeft);
                    }

                    _ => {}
                },
                Direction::Bottom => match ahead_direction {
                    Direction::Bottom => {
                        move_coordinate(&mut current_knot, Direction::Bottom);
                    }

                    Direction::BottomRight => {
                        move_coordinate(&mut current_knot, Direction::BottomRight);
                    }

                    Direction::BottomLeft => {
                        move_coordinate(&mut current_knot, Direction::BottomLeft);
                    }

                    _ => {}
                },
                Direction::BottomRight => match ahead_direction {
                    Direction::Bottom => {
                        move_coordinate(&mut current_knot, Direction::BottomRight);
                    }

                    Direction::BottomRight => {
                        move_coordinate(&mut current_knot, Direction::BottomRight);
                    }

                    Direction::BottomLeft => {
                        move_coordinate(&mut current_knot, Direction::Bottom);
                    }

                    Direction::TopRight => {
                        move_coordinate(&mut current_knot, Direction::Right);
                    }

                    Direction::Right => {
                        move_coordinate(&mut current_knot, Direction::BottomRight);
                    }

                    _ => {}
                },
                Direction::BottomLeft => match ahead_direction {
                    Direction::Bottom => {
                        move_coordinate(&mut current_knot, Direction::BottomLeft);
                    }

                    Direction::BottomLeft => {
                        move_coordinate(&mut current_knot, Direction::BottomLeft);
                    }

                    Direction::BottomRight => {
                        move_coordinate(&mut current_knot, Direction::Bottom);
                    }

                    Direction::TopLeft => {
                        move_coordinate(&mut current_knot, Direction::Left);
                    }

                    Direction::Left => {
                        move_coordinate(&mut current_knot, Direction::BottomLeft);
                    }

                    _ => {}
                },

                _ => {}
            }

            if knots.len() > index + 1 {
                ahead_direction = get_direction(knots[index], knots[index + 1]);
            }

            move_direction = get_direction(current_knot, *knot);

            *knot = current_knot;
        }
    }
}

fn main() {
    let input = include_str!("input.prod");

    // part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let instructions = input
        .lines()
        .map(|line| {
            line.parse::<Instruction>()
                .expect("Failed to parse instruction")
        })
        .collect::<Vec<Instruction>>();

    let mut rope = Rope {
        knots: vec![(0, 0), (0, 0)],
    };

    let seen: Vec<(i32, i32)> = instructions
        .iter()
        .flat_map(|instruction| {
            let mut seen = Vec::new();
            for _ in 0..instruction.steps {
                rope.move_direction(instruction);
                seen.push(*rope.knots.last().unwrap());
            }

            seen
        })
        .collect();

    let seen_set = seen
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<(i32, i32)>>();

    println!("Part 1: {:?}", seen_set.len());
}

fn part_2(input: &str) {
    let instructions = input
        .lines()
        .map(|line| {
            line.parse::<Instruction>()
                .expect("Failed to parse instruction")
        })
        .collect::<Vec<Instruction>>();

    let mut rope = Rope {
        knots: vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ],
    };

    let seen: Vec<(i32, i32)> = instructions
        .iter()
        .flat_map(|instruction| {
            let mut seen = Vec::new();
            for _ in 0..instruction.steps {
                rope.move_direction(instruction);
                seen.push(*rope.knots.last().unwrap());
            }

            seen
        })
        .collect();

    let seen_set = seen
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<(i32, i32)>>();

    println!("Part 2: {:?}", seen_set.len());
}

fn get_direction(coordinate_one: (i32, i32), coordinate_two: (i32, i32)) -> Direction {
    match coordinate_one.0.cmp(&coordinate_two.0) {
        Ordering::Less => match coordinate_one.1.cmp(&coordinate_two.1) {
            Ordering::Less => Direction::BottomLeft,
            Ordering::Equal => Direction::Left,
            Ordering::Greater => Direction::TopLeft,
        },
        Ordering::Equal => match coordinate_one.1.cmp(&coordinate_two.1) {
            Ordering::Less => Direction::Bottom,
            Ordering::Equal => Direction::Overlap,
            Ordering::Greater => Direction::Top,
        },
        Ordering::Greater => match coordinate_one.1.cmp(&coordinate_two.1) {
            Ordering::Less => Direction::BottomRight,
            Ordering::Equal => Direction::Right,
            Ordering::Greater => Direction::TopRight,
        },
    }
}

fn move_coordinate(coordinate: &mut (i32, i32), direction: Direction) {
    match direction {
        Direction::Top => coordinate.1 += 1,
        Direction::Left => coordinate.0 -= 1,
        Direction::Right => coordinate.0 += 1,
        Direction::Bottom => coordinate.1 -= 1,
        Direction::TopLeft => {
            coordinate.0 -= 1;
            coordinate.1 += 1;
        }
        Direction::TopRight => {
            coordinate.0 += 1;
            coordinate.1 += 1;
        }
        Direction::BottomLeft => {
            coordinate.0 -= 1;
            coordinate.1 -= 1;
        }
        Direction::BottomRight => {
            coordinate.0 += 1;
            coordinate.1 -= 1;
        }
        Direction::Overlap => {}
    }
}
