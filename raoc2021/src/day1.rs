use lib::*;

fn day1a() -> u32 {
    let mut count = 0;
    let mut prev: Option<u32> = None;

    for l in read_lines("../inputs/1.txt")
        .expect("no read")
        .map(|l| l.expect("no line").parse::<u32>().expect("no parse"))
    {
        if let Some(p) = prev {
            if p < l {
                count += 1;
            }
        }
        prev = Some(l)
    }
    count
}

fn day1b() -> u32 {
    let mut count = 0;

    let depths: Vec<u32> = read_lines("../inputs/1.txt")
        .expect("no read")
        .map(|l| l.expect("no line").parse::<u32>().expect("no parse"))
        .collect();

    for i in 0..depths.len() - 3 {
        let a = depths[i] + depths[i + 1] + depths[i + 2];
        let b = depths[i + 1] + depths[i + 2] + depths[i + 3];
        if b > a {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("Day 1 (part 1): {}", day1a());
    println!("Day 1 (part 2): {}", day1b());
}
