use std::fs::read_to_string;

fn main() {
    let input = read_file("input.txt");
    println!("Puzzle 1: {:?}", puzzle1(&input));
    println!("Puzzle 2: {:?}", puzzle2(&input));
}

fn puzzle1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let opponent = line.chars().nth(0).unwrap() as u32 - 'A' as u32;
            let player = line.chars().nth(2).unwrap() as u32 - 'X' as u32;

            game_score(player, opponent) + player + 1
        })
        .sum()
}

// computes the player's score for a round, given the player and opponent moves
fn game_score(player: u32, opponent: u32) -> u32 {
    match player {
        p if p == (opponent + 1) % 3 => 6,
        p if p == opponent => 3,
        _ => 0,
    }
}

fn puzzle2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let opponent = line.chars().nth(0).unwrap() as u32 - 'A' as u32;
            let outcome = line.chars().nth(2).unwrap() as u32 - 'X' as u32;
            let player = move_for_outcome(opponent, outcome);

            outcome * 3 + player + 1
        })
        .sum()
}

// computes the move the player should make to get the given outcome, given the opponent's move
fn move_for_outcome(opponent: u32, outcome: u32) -> u32 {
    match outcome {
        0 => (opponent + 2) % 3,
        1 => opponent,
        2 => (opponent + 1) % 3,
        _ => panic!("Invalid outcome"),
    }
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
