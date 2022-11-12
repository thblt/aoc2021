use lib::*;

use std::str::FromStr;

enum Day2Direction {
    Forward,
    Down,
    Up,
}

struct Day2A {
    direction: Day2Direction,
    value: u32,
}

impl FromStr for Day2A {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let direction = match s.next().ok_or(())? {
            "forward" => Some(Day2Direction::Forward),
            "up" => Some(Day2Direction::Up),
            "down" => Some(Day2Direction::Down),
            &_ => None,
        }
        .ok_or(())?;
        let value = s.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;

        Ok(Day2A { direction, value })
    }
}

fn day2a() -> u32 {
    let mut h = 0;
    let mut d = 0;

    for l in read_lines("../inputs/2.txt")
        .expect("no read")
        .map(|l| l.expect("no line").parse::<Day2A>().expect("no parse"))
    {
        match l.direction {
            Day2Direction::Forward => h += l.value,
            Day2Direction::Down => d += l.value,
            Day2Direction::Up => d -= l.value,
        }
    }
    h * d
}

fn day2b() -> u32 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for l in read_lines("../inputs/2.txt")
        .expect("no read")
        .map(|l| l.expect("no line").parse::<Day2A>().expect("no parse"))
    {
        match l.direction {
            Day2Direction::Forward => {
                h += l.value;
                d += aim * l.value;
            }
            Day2Direction::Down => aim += l.value,
            Day2Direction::Up => aim -= l.value,
        }
    }
    h * d
}

fn main() {
    println!("Day 2 (part 1): {}", day2a());
    println!("Day 2 (part 2): {}", day2b());
}
