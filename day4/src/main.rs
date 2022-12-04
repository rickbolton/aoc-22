use std::fs;

#[derive(Debug, PartialEq)]
struct Elf {
    min: i32,
    max: i32,
}

#[derive(Debug, PartialEq)]
struct Pair {
    elves: (Elf, Elf),
}

impl Pair {
    fn has_overlap(&self) -> bool {
        let (one, two) = &self.elves;

        let one_overlaps_two = one.min <= two.min && one.max >= two.max;
        let two_overlaps_one = two.min <= one.min && two.max >= one.max;

        one_overlaps_two || two_overlaps_one
    }

    fn has_partial_overlap(&self) -> bool {
        let (one, two) = &self.elves;

        one.min <= two.max && two.min <= one.max
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total = input
        .lines()
        .map(parse_pair)
        .filter(|p| p.has_overlap())
        .count();

    println!("{:?}", total);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total = input
        .lines()
        .map(parse_pair)
        .filter(|p| p.has_partial_overlap())
        .count();

    println!("{:?}", total);
}

fn parse_elves(input: Vec<&str>) -> (Elf, Elf) {
    let range: Vec<&str> = input[0].split('-').collect();
    let elf_one = Elf {
        min: range[0].parse::<i32>().unwrap(),
        max: range[1].parse::<i32>().unwrap(),
    };

    let range: Vec<&str> = input[1].split('-').collect();
    let elf_two = Elf {
        min: range[0].parse::<i32>().unwrap(),
        max: range[1].parse::<i32>().unwrap(),
    };

    (elf_one, elf_two)
}

fn parse_pair(input: &str) -> Pair {
    Pair {
        elves: parse_elves(input.split(',').collect()),
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_pair, Elf, Pair};

    const INPUT: &str = r#"2-4,6-8"#;

    #[test]
    fn can_parse_group_from_input() {
        let elf_one = Elf { min: 2, max: 4 };
        let elf_two = Elf { min: 6, max: 8 };
        assert_eq!(
            parse_pair(INPUT),
            Pair {
                elves: (elf_one, elf_two)
            }
        )
    }

    const OVERLAP_INPUT: &str = r#"2-4,2-8"#;
    #[test]
    fn pair_check_for_overlapping_elves_assignment() {
        let pair = parse_pair(OVERLAP_INPUT);

        assert!(pair.has_overlap());
    }
}
