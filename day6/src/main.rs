use std::fs;

fn main() {
    part_1();
    part_2();
}

fn find_marker(input: String, distinct_len: usize) -> usize {
    let position = input
        .as_bytes()
        .windows(distinct_len)
        .position(move |set| {
            let mut data: u32 = 0;
            for &c in set {
                let prev = data;

                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            return true;
        })
        .unwrap();

    println!("distinct: {} position:{}", distinct_len, position);

    position + distinct_len
}

fn part_1() {
    let input = fs::read_to_string("input.txt").expect("file not found");

    let idx = find_marker(input, 4);
    println!("the idx is: {}", idx);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").expect("file not found");

    let idx = find_marker(input, 14);
    println!("the idx is: {}", idx);
}

#[cfg(test)]
mod test {
    use crate::find_marker;

    const INPUT: &'static str = "asnsbnnlblnoroyuac";

    #[test]
    fn test_find_marker() {
        assert_eq!(find_marker(INPUT.to_string(), 4), 12);
    }
}
