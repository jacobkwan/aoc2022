use std::fs::read_to_string;
fn main() {
    let initial_state = read_file("initial.txt");
    let stacks = construct_initial_stacks(&initial_state);
    let moves = read_file("input.txt");
    println!("Part 1: {}", part1(&stacks, &moves));
    println!("Part 2: {}", part2(&stacks, &moves));
}

fn part2(stacks: &Vec<Vec<char>>, moves: &str) -> String {
    let mut stacks = stacks.clone();
    for line in moves.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let num_crates_to_move = line[1].parse::<usize>().unwrap();
        let from_stack = line[3].parse::<usize>().unwrap() - 1;
        let to_stack = line[5].parse::<usize>().unwrap() - 1;

        let from_stack_len = stacks[from_stack].len();
        // can use `vec.drain` too
        let mut crates_to_move = stacks[from_stack].split_off(from_stack_len - num_crates_to_move);
        stacks[to_stack].append(&mut crates_to_move);
    }

    // return the top most crate of each stack at the end
    stacks
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn part1(stacks: &Vec<Vec<char>>, moves: &str) -> String {
    let mut stacks = stacks.clone();
    for line in moves.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let num_crates_to_move = line[1].parse::<u32>().unwrap();
        let from_stack = line[3].parse::<usize>().unwrap() - 1;
        let to_stack = line[5].parse::<usize>().unwrap() - 1;

        for _ in 0..num_crates_to_move {
            let crate_to_move = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(crate_to_move);
        }
    }

    // return the top most crate of each stack at the end
    stacks
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn construct_initial_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    // populate stacks from bottom up, row by row
    for line in input.lines().rev() {
        for col in 0..9 {
            let char = line.chars().nth(col * 4 + 1).unwrap();
            if char.is_whitespace() {
                continue;
            }
            stacks[col].push(char);
        }
    }
    stacks
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
