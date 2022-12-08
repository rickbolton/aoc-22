use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.prod").unwrap();

    // parse numbers from each line
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part_1(&grid);
    part_2(&grid);
}

fn part_1(grid: &[Vec<u32>]) {
    let mut count = 0;
    let rows = grid.len();
    let columns = grid[0].len();

    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            let n = grid[y][x];
            // From left
            let vis = (0..x).all(|x1| grid[y][x1] < n)
                // From right
                || (x+1..rows).all(|x1| grid[y][x1] < n)
                // From top
                || (0..y).all(|y1| grid[y1][x] < n)
                // From bottom
                || (y+1..columns).all(|y1| grid[y1][x] < n);

            if vis {
                count += 1;
            }
        }
    }

    let outer_cells = (columns + rows - 2) * 2;

    println!("Part 1: {}", outer_cells + count);
}

fn part_2(grid: &[Vec<u32>]) {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut scenic_scores: Vec<i32> = vec![];
    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            let n = grid[y][x];
            let mut left_score = 0;
            let mut right_score = 0;
            let mut front_score = 0;
            let mut back_score = 0;

            for x1 in (0..x).rev() {
                left_score += 1;
                if grid[y][x1] >= n {
                    break;
                }
            }

            for x1 in x + 1..rows {
                right_score += 1;
                if grid[y][x1] >= n {
                    break;
                }
            }

            for y1 in (0..y).rev() {
                front_score += 1;
                if grid[y1][x] >= n {
                    break;
                }
            }

            for y1 in y + 1..columns {
                back_score += 1;
                if grid[y1][x] >= n {
                    break;
                }
            }

            let scenic_score = left_score * right_score * front_score * back_score;
            scenic_scores.push(scenic_score);
        }
    }

    scenic_scores
        .iter()
        .max()
        .map(|m| println!("Part 2: {:#?}", m));
}
