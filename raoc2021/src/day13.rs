use lib::*;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Fold {
    axis: FoldAxis,
    i: isize,
}

fn cut<T: FromStr>(s: &str, delim: &str) -> Option<(T, T)> {
    if let Some((a, b)) = s.to_string().split_once(delim) {
        Some((a.parse::<T>().ok()?, b.parse::<T>().ok()?))
    } else {
        None
    }
}

fn print_bool(b: &bool) -> String {
    if *b { "#" } else { "." }.to_string()
}

fn fold(paper: &mut Vec2D<bool>, fold: &Fold) {
    let i = fold.i;
    if fold.axis == FoldAxis::X {
        // Horizontal
        let count: isize = min(i + 1, paper.width() as isize - i);
        for d in 1..count {
            for y in 0..paper.height() as isize {
                // println!("i={}, d={}, y={}, count={}", i, d, y, count);
                paper[(i - d, y)] |= paper[(i + d, y)];
                paper[(i + d, y)] = false;
            }
        }
    } else if fold.axis == FoldAxis::Y {
        // Vertical
        let count: isize = min(i + 1, paper.height() as isize - i);
        for d in 1..count {
            for x in 0..paper.width() as isize {
                paper[(x, i - d)] |= paper[(x, i + d)];
                paper[(x, i + d)] = false;
            }
        }
    }
}

fn main() {
    // First iteration to get dimensions
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in read_lines("../inputs/13.txt").unwrap() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let (x, y) = cut::<usize>(&line, ",").unwrap();
        width = max(x + 1, width);
        height = max(y + 1, height);
    }

    println!("Init for wh {}×{}", width, height);
    let mut paper: Vec2D<bool> = Vec2D::new(width, height, false);
    let mut folds: Vec<Fold> = vec![];

    let mut step1 = true;
    for line in read_lines("../inputs/13.txt").unwrap() {
        let line = line.unwrap();

        if line.is_empty() {
            step1 = false;
            continue;
        }
        if step1 {
            // Read dots
            if let Some((x, y)) = cut::<isize>(&line, ",") {
                paper[(x, y)] = true;
            } else {
                panic!("no parse.");
            }
        } else {
            // Read folds
            let axis = if line.chars().nth(11).unwrap() == 'x' {
                FoldAxis::X
            } else {
                FoldAxis::Y
            };
            let i = line.rsplit("=").next().unwrap().parse::<isize>().unwrap();
            folds.push(Fold { axis, i })
        }
    }
    fold(&mut paper, &folds[0]);
    println!("Part 1: {}", &paper.vec.iter().filter(|b| **b).count());
    for fld in folds.iter().skip(1) {
        println!("Folding {:?}", &fld);
        fold(&mut paper, &fld);
    }

    paper.draw_with(&print_bool);
    // This is VERY hacky
    for x in 0..200 {
        for y in 0..200 {
            print!("{}", if paper[(x, y)] { "▉" } else { " " })
        }
        println!();
    }
}
