use std::fs;
use std::cmp::{max,min};

fn distance(a: u32, b: u32) -> u32 {
    max(a, b) - min(a,b)
}

fn cost_b(a: u32, b: u32) -> u32 {
    let dist = distance(a,b) as f64;

    let ret = (dist.powf(2.0) / 2.0) + (dist / 2.0);
    ret as u32
}

fn part_a() -> (u32,u32) {
    // let raw = String::from("16,1,2,0,4,2,7,1,2,14");
    let raw = fs::read_to_string("../inputs/7.txt").unwrap();
    let positions: Vec<u32> = raw.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut best_a: Option<u32> = None;
    let mut best_b: Option<u32> = None;
    for cand in *min..=*max {
        // let mut cost = 0;
        let cost_a: u32 = positions.iter().map(|x| distance(*x, cand)).sum();
        let cost_b: u32 = positions.iter().map(|x| cost_b(*x, cand)).sum();
        if let Some (x) = best_a {
            if x > cost_a {
                best_a = Some(cost_a)
            }
        } else {
            best_a = Some(cost_a)
        }

        if let Some (x) = best_b {
            if x > cost_b {
                best_b = Some(cost_b)
            }
        } else {
            best_b = Some(cost_b)
        }


    }

    (best_a.unwrap(), best_b.unwrap())
}

fn main() {
    println!("Part A: {:?}", part_a());
    // println!("Part B: {}", part_b());
}
