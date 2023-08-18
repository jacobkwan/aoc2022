use std::fs::read_to_string;

fn main() {
    let input = read_file("input.txt");
    println!("Puzzle 1: {:?}", puzzle1(&input));
    println!("Puzzle 2: {:?}", puzzle2(&input));
}

fn puzzle1(input: &str) -> u32 {
    // map each elf's items to a sum and return the max
    input
        .split("\n\n")
        .map(|elf_items| {
            elf_items
                .split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn puzzle2(input: &str) -> u32 {
    let mut calories_by_elf = input
        .split("\n\n")
        .map(|elf_items| {
            elf_items
                .split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();

    // sort in descending and return largest 3
    calories_by_elf.sort_unstable_by(|a, b| b.cmp(a));

    calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2]
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
