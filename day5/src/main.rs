use std::fs::read_to_string;
fn main() {
    let initial_state = read_file("initial.txt");
    let stacks = construct_initial_stacks(&initial_state);
    let moves = read_file("input.txt");
    println!("Part 1: {}", part1(&stacks, &moves));
    println!("Part 1 (FP): {}", part1_fp(&stacks, &moves));
    println!("Part 2: {}", part2(&stacks, &moves));
    println!("Part 2 (FP): {}", part2_fp(&stacks, &moves));
}

fn part2_fp(stacks: &[Vec<char>], moves: &str) -> String {
    moves
        .lines()
        .fold(stacks.to_owned(), |mut stacks, curr_move| {
            let curr_move = curr_move.split_whitespace().collect::<Vec<&str>>();
            let num_crates_to_move = curr_move[1].parse::<usize>().unwrap();
            let from_stack = curr_move[3].parse::<usize>().unwrap() - 1;
            let to_stack = curr_move[5].parse::<usize>().unwrap() - 1;

            let from_stack = &mut stacks[from_stack];
            let crates_to_move = from_stack.drain(from_stack.len() - num_crates_to_move..);

            // need this so rust will drop the mutable reference to `stacks` from line 41 that is moved
            // into `crates_to_move` in line 42
            let crates_to_move = crates_to_move.collect::<Vec<char>>();
            stacks[to_stack].extend(crates_to_move);
            stacks
        })
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn part2(stacks: &[Vec<char>], moves: &str) -> String {
    let mut stacks = stacks.to_owned();
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

fn part1_fp(stacks: &[Vec<char>], moves: &str) -> String {
    moves
        .lines()
        .fold(stacks.to_owned(), |mut stacks, curr_move| {
            let curr_move = curr_move.split_whitespace().collect::<Vec<&str>>();
            let num_crates_to_move = curr_move[1].parse::<usize>().unwrap();
            let from_stack = curr_move[3].parse::<usize>().unwrap() - 1;
            let to_stack = curr_move[5].parse::<usize>().unwrap() - 1;

            let from_stack = &mut stacks[from_stack];
            let crates_to_move = from_stack
                .drain(from_stack.len() - num_crates_to_move..)
                .rev()
                .collect::<Vec<char>>();

            // need this so rust will drop the mutable reference to `stacks` from line 41 that is moved
            // into `crates_to_move` in line 42
            // let crates_to_move = crates_to_move.collect::<Vec<char>>();
            stacks[to_stack].extend(crates_to_move);
            stacks
        })
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}

fn part1(stacks: &[Vec<char>], moves: &str) -> String {
    let mut stacks = stacks.to_owned();
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
    (0..9)
        .map(|col| {
            input
                .lines()
                .rev()
                .filter_map(|row| {
                    let char = row.chars().nth(col * 4 + 1).unwrap();
                    (!char.is_whitespace()).then_some(char)
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()

    // imperative way
    // // populate stacks from bottom up, row by row
    // let mut stacks = vec![Vec::new(); 9];
    // for line in input.lines().rev() {
    //     for col in 0..9 {
    //         let char = line.chars().nth(col * 4 + 1).unwrap();
    //         if char.is_whitespace() {
    //             continue;
    //         }
    //         stacks[col].push(char);
    //     }
    // }
    // stacks
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
