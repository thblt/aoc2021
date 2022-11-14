use lib::*;

fn score_line(line: &str) -> u32 {
    let mut stack: Vec<char> = vec!();
    for c in line.chars() {
        match c {
            '{' => stack.push(c),
            '[' => stack.push(c),
            '(' => stack.push(c),
            '<' => stack.push(c),
            '}' => if stack.pop().unwrap() != '{' { return 1197; },
            ']' => if stack.pop().unwrap() != '[' { return 57; },
            '>' => if stack.pop().unwrap() != '<' { return 25137; },
            ')' => if stack.pop().unwrap() != '(' { return 3; },
            _ => panic!("Very bad input"),
        }
    }
    0
}

fn main() {
    let score =
        read_lines("../inputs/10.txt")
        .unwrap()
        .map(|line| score_line(&line.unwrap()))
        .sum::<u32>();

    println!("{}", score);

    // Part 2

    let score =
        read_lines("../inputs/10.txt")
        .unwrap()
        .filter(|line| score_line(&line.unwrap()) == 0)


}
