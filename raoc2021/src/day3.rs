use ::lib::*;

#[derive(Debug)]
struct Day3FoldState {
    count: u32,
    ones: Vec<u32>,
}
/// The fold function for counting ones
fn day3fold<S: Into<String>>(
    Day3FoldState {
        mut count,
        mut ones,
    }: Day3FoldState,
    line: S,
) -> Day3FoldState {
    let line = line.into();
    count += 1;
    for i in 0..line.len() {
        if ones.len() < i + 1 {
            ones.push(0)
        }

        if line.chars().nth(i) == Some('1') {
            ones[i] += 1;
        }
    }
    Day3FoldState { count, ones }
}

/// remove elements from list, in place, whose value at position `i`
/// isn't the most or least common.  Don't do anything is the list is
/// of len 1.
fn day3filter(list: &mut Vec<Vec<bool>>, i: usize, strategy: bool) {
    if list.len() == 1 {
        return;
    }

    let count = list.iter().filter(|v| v[i]).count();
    let mcv = (count * 2) >= list.len();

    // println!("At position {}, count is {} out of {}, so mcv is {} (strategy: {}).",
    //          i,
    //          count,
    //          list.len(),
    //          mcv,
    //          strategfy);

    // Double comparison: compare x[i] to mcv, then compare the bool
    // result to strategy to reverse.
    list.retain(|x| (x[i] == mcv) == strategy);
}

fn day3a() -> u32 {
    let init = Day3FoldState {
        count: 0,
        ones: Vec::new(),
    };
    let result = read_lines("../inputs/3.txt")
        .expect("no read")
        .map(|s| s.unwrap())
        .fold(init, day3fold);

    let mut gamma = 0;

    let mut power = u32::pow(2, result.ones.len() as u32 - 1);
    let max = u32::pow(2, result.ones.len() as u32) - 1;

    for digit in result.ones.iter() {
        if digit * 2 > result.count {
            // One is more frequent.
            gamma += power
        }
        power /= 2;
    }
    let epsilon = max - gamma;
    println!("Gamma: {} Epsilon: {}", gamma, epsilon);
    gamma * epsilon
}

/// Parse a binary number expressed as a vector of booleans.
fn parse_bin_bools(n: &[bool]) -> u32 {
    let mut power = u32::pow(2, n.len() as u32 - 1);
    let mut ret = 0;

    for digit in n.iter() {
        if *digit {
            ret += power
        }
        power /= 2;
    }
    ret
}

fn day3b() -> u32 {
    let mut cands_g: Vec<Vec<bool>> = read_lines("../inputs/3.txt")
        .expect("no read")
        .map(|s| s.unwrap().chars().map(|x| x == '1').collect())
        .collect();

    let mut cands_s = cands_g.clone();

    for i in 0..cands_s[0].len() {
        day3filter(&mut cands_g, i, true);
        day3filter(&mut cands_s, i, false);
        // println!(
        //     "Iteration {}, {} generator cand(s) and {} scrubber cand(s)",
        //     i,
        //     cands_g.len(),
        //     cands_s.len()
        // );

        // println!("FOR GENERATOR");
        // for cand in &cands_g {
        //     println!("- {}", bools_to_bin_string(&cand));
        // }
    }

    let val_g = parse_bin_bools(&cands_g[0]);
    let val_s = parse_bin_bools(&cands_s[0]);
    println!(
        "Done. g={} ({}), s={} ({})",
        bools_to_bin_string(&cands_g[0]),
        val_g,
        bools_to_bin_string(&cands_s[0]),
        val_s
    );
    val_g * val_s
}

fn main() {
    println!("Day 3 (part 1): {}", day3a());
    println!("Day 3 (part 2): {}", day3b());
}
