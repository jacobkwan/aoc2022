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
    if player == (opponent + 1) % 3 {
        return 6;
    }
    if player == opponent {
        return 3;
    }
    0
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
    if outcome == 0 {
        return (opponent + 2) % 3;
    }
    if outcome == 1 {
        return opponent;
    }
    (opponent + 1) % 3
}

fn read_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}
