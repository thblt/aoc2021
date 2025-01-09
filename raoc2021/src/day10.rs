use lib::*;

fn score_line_a(line: &str) -> u32 {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '{' => stack.push(c),
            '[' => stack.push(c),
            '(' => stack.push(c),
            '<' => stack.push(c),
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return 1197;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return 57;
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return 25137;
                }
            }
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return 3;
                }
            }
            _ => panic!("Very bad input"),
        }
    }
    0
}

fn score_line_b(line: &str) -> u64 {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '{' => stack.push(c),
            '[' => stack.push(c),
            '(' => stack.push(c),
            '<' => stack.push(c),
            '}' => {
                stack.pop();
            }
            ']' => {
                stack.pop();
            }
            '>' => {
                stack.pop();
            }
            ')' => {
                stack.pop();
            }
            _ => panic!("Very bad input"),
        }
    }
    let mut score: u64 = 0;
    for c in stack.into_iter().rev() {
        // print!("{}",c);
        score *= 5;
        score += match c {
            '{' => 3,
            '[' => 2,
            '(' => 1,
            '<' => 4,
            _ => panic!("Very bad input"),
        }
    }
    score
}

fn main() {
    let score = read_lines("../inputs/10.txt")
        .unwrap()
        .map(|line| score_line_a(&line.unwrap()))
        .sum::<u32>();

    println!("{}", score);

    // Part 2

    let mut score: Vec<u64> = read_lines("../inputs/10.txt")
        .unwrap()
        .filter(|line| score_line_a(&line.as_ref().unwrap()) == 0)
        .map(|line| score_line_b(&line.unwrap()))
        .collect();

    score.sort();
    println!("{:?}", score[score.len() / 2]);
}
