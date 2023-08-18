use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_file("input.txt");
    println!("{:?}", puzzle1(&input));
    println!("{:?}", puzzle2(&input));
}

fn puzzle1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|rucksack| {
            let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
            let first_half_letters_set = first_half.chars().collect::<HashSet<char>>();

            // find the common letter between first_half and second_half
            second_half
                .chars()
                .find(|c| first_half_letters_set.contains(c))
                .map_or(0, priority)
        })
        .sum()
}

fn priority(char: char) -> u32 {
    if char.is_lowercase() {
        char as u32 - 'a' as u32 + 1
    } else {
        char as u32 - 'A' as u32 + 27
    }
}

fn puzzle2(input: &str) -> u32 {
    let mut priorities_sum = 0;
    let num_groups = input.lines().count() / 3;
    for group_idx in 0..num_groups {
        let first_elf_set: HashSet<char> =
            HashSet::from_iter(input.lines().nth(group_idx * 3).unwrap().chars());
        let second_elf_set: HashSet<char> =
            HashSet::from_iter(input.lines().nth(group_idx * 3 + 1).unwrap().chars());
        let third_elf_set: HashSet<char> =
            HashSet::from_iter(input.lines().nth(group_idx * 3 + 2).unwrap().chars());

        // find the common letter
        for char in first_elf_set {
            if second_elf_set.contains(&char) && third_elf_set.contains(&char) {
                priorities_sum += priority(char);
                break;
            }
        }
    }
    priorities_sum
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
