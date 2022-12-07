use std::{collections::HashMap, fs, vec};

#[derive(Debug)]
struct Directory<'a> {
    path: &'a str,
    files: Vec<File>,
    children: HashMap<&'a str, Directory<'a>>,
}

fn sum_dir(dir: &Directory) -> usize {
    let mut tot = 0;
    for f in &dir.files {
        tot += f.size as usize;
    }

    for (_, d) in &dir.children {
        tot += sum_dir(&d);
    }

    tot
}

fn flatten<'a>(dir: &'a Directory) -> Vec<&'a Directory<'a>> {
    let mut dirs = vec![];
    dirs.push(dir.clone());

    for (_, d) in &dir.children {
        dirs.append(flatten(d).as_mut());
    }

    dirs
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut root = Directory {
        path: "",
        files: Vec::new(),
        children: HashMap::new(),
    };

    let mut working_dir: Vec<&str> = vec![""];

    let lines = input
        .lines()
        .filter(|l| !l.starts_with("$ ls") && !l.starts_with("dir"))
        .collect::<Vec<&str>>();

    for line in lines {
        if line.contains("$ cd ..") {
            working_dir.pop();
            continue;
        }

        if line.starts_with("$ cd ") {
            let path = line.split_whitespace().last().unwrap();
            if path != "/" {
                working_dir.push(path);
            }
            continue;
        }

        let mut dir = &mut root;

        for p in working_dir.clone().into_iter() {
            if p == "" {
                continue;
            }

            dir = dir.children.entry(p).or_insert(Directory {
                path: p,
                children: HashMap::new(),
                files: Vec::new(),
            })
        }

        let mut parts = line.split_whitespace();

        let first_part = parts.next().unwrap();
        let second_part = parts.next().unwrap();

        let size = first_part.parse::<u64>().unwrap();
        let name = second_part;
        dir.files.push(File {
            name: name.to_string(),
            size,
        });
    }

    // flatten and print total for sum_dir that is less than 100000
    let total = flatten(&root)
        .iter()
        .filter(|d| sum_dir(&d) < 100000)
        .map(|d| sum_dir(&d))
        .sum::<usize>();

    println!("Part 1: {:#?}", total);
}

fn part_2(input: &str) {
    let mut root = Directory {
        path: "",
        files: Vec::new(),
        children: HashMap::new(),
    };

    let mut working_dir: Vec<&str> = vec![""];

    let lines = input
        .lines()
        .filter(|l| !l.starts_with("$ ls") && !l.starts_with("dir"))
        .collect::<Vec<&str>>();

    for line in lines {
        if line.contains("$ cd ..") {
            working_dir.pop();
            continue;
        }

        if line.starts_with("$ cd ") {
            let path = line.split_whitespace().last().unwrap();
            if path != "/" {
                working_dir.push(path);
            }
            continue;
        }

        let mut dir = &mut root;

        for p in working_dir.clone().into_iter() {
            if p == "" {
                continue;
            }

            dir = dir.children.entry(p).or_insert(Directory {
                path: p,
                children: HashMap::new(),
                files: Vec::new(),
            })
        }

        let mut parts = line.split_whitespace();

        let first_part = parts.next().unwrap();
        let second_part = parts.next().unwrap();

        let size = first_part.parse::<u64>().unwrap();
        let name = second_part;
        dir.files.push(File {
            name: name.to_string(),
            size,
        });
    }

    let free_space = 70000000 - sum_dir(&root);
    let space_required = 30000000 - free_space;

    println!("Free space: {:#?}", free_space);
    println!("Space required: {:#?}", space_required);

    // loop over all flatten dirs that size is larger than space_required and find the smallest
    flatten(&root)
        .iter()
        .filter(|d| sum_dir(&d) >= space_required)
        .map(|d| sum_dir(&d))
        .min()
        .map(|m| println!("Part 2: {:#?}", m));
}
