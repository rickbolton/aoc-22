use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();

    let total = lines
        .iter()
        .filter(|b| !b.is_empty())
        .fold(0, |acc, b| acc + priority_from_char(item_type_from_line(b)));

    println!("{:?}", total);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<&str>>();

    let total = lines.chunks(3).fold(0, |acc, b| {
        if !b[0].is_empty() {
            acc + priority_from_char(item_type_from_group((b[0], b[1], b[2])))
        } else {
            acc
        }
    });

    println!("{:?}", total);
}

fn priority_from_char(char: char) -> i32 {
    if char.is_ascii_lowercase() {
        (char as u8 - b'a' + 1).into()
    } else {
        (char as u8 - b'A' + 27).into()
    }
}

fn item_type_from_line(line: &str) -> char {
    let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

    for fc in first_compartment.chars() {
        for sc in second_compartment.chars() {
            if fc == sc {
                return fc;
            }
        }
    }

    panic!("unable to find item type")
}

fn item_type_from_group((elf_one, elf_two, elf_three): (&str, &str, &str)) -> char {
    for e1 in elf_one.chars() {
        for e2 in elf_two.chars() {
            for e3 in elf_three.chars() {
                if e1 == e2 && e1 == e3 {
                    return e1;
                }
            }
        }
    }

    panic!("unable to find item type")
}

#[cfg(test)]
mod tests {
    use crate::{item_type_from_group, item_type_from_line, priority_from_char};

    #[test]
    fn test_priority_from_char() {
        assert_eq!(priority_from_char('a'), 1);
        assert_eq!(priority_from_char('p'), 16);
        assert_eq!(priority_from_char('A'), 27);
    }

    #[test]
    fn test_item_type_from_line() {
        assert_eq!(item_type_from_line("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(item_type_from_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
    }

    #[test]
    fn test_item_type_from_group() {
        assert_eq!(
            item_type_from_group((
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            )),
            'r'
        );

        assert_eq!(
            item_type_from_group((
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            'Z'
        );
    }
}
