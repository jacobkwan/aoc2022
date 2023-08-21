#[macro_use]
extern crate scan_fmt;
use std::fs::read_to_string;
fn main() {
    let input = read_file("input.txt");
    println!("Puzzle 1: {:?}", puzzle1(&input));
    println!("Puzzle 2: {:?}", puzzle2(&input));
}

fn puzzle1(input: &str) -> usize {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap())
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count()
}

fn puzzle2(input: &str) -> usize {
    input
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap())
        .filter(|(a, b, c, d)| (a <= c && c <= b) || (a >= c && a <= d))
        .count()
}

fn read_file(file_path: &str) -> String {
    let input = read_to_string(file_path).unwrap().trim().to_string();
    input
}
