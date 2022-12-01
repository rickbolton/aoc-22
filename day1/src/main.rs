use std::fs;

struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn total(&self) -> i32 {
        let mut total = 0;

        self.calories.clone().into_iter().for_each(|c| {
            total += c;
        });

        total
    }
}

struct Elves {
    elves: Vec<Elf>,
}

impl Elves {
    fn new() -> Elves {
        Elves { elves: vec![] }
    }

    fn add_elf(&mut self, elf: Elf) {
        self.elves.push(elf);
        self.sort();
    }

    fn sort(&mut self) {
        self.elves.sort_by_key(|e| e.total());
        self.elves.reverse();
    }

    fn top_1(&self) {
        println!("top 1 : {:?}", self.elves[0].total());
    }

    fn top_3(&self) {
        println!(
            "top 3 : {:?}",
            (self.elves[0].total() + self.elves[1].total() + self.elves[2].total())
        );
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let batches = input.split("\n\n").collect::<Vec<&str>>();

    let mut santas_elves = Elves::new();

    for c in batches {
        let calories = c
            .split('\n')
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        let elf = Elf { calories };

        santas_elves.add_elf(elf);
    }

    santas_elves.top_1();
    santas_elves.top_3();
}
