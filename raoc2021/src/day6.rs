use std::fs;

fn make_state(raw: &[u64]) -> Vec<u64> {
    let mut ret = vec![0,0,0,0,0,0,0,0,0];
    for fish in raw {
        ret[*fish as usize] += 1;
    }
    ret
}

fn part_b(days: u64) -> u64 {
    // The interesting part about part 2 is that it's exactly the same
    // as A, but you can't bruteforce your way to a result --- you're
    // supposed to compute 256 days, not 80.

    // let raw = String::from("3,4,3,1,2");
    let raw = fs::read_to_string("../inputs/6.txt").unwrap();

    let mut state = make_state(&raw
                               .split(",")
                               .map(|x|
                                    x.trim().parse::<u64>().unwrap())
                               .collect::<Vec<u64>>());

    for day in 1..=days {
        let zeros = state[0];
        for i in 1..9 {
            // println!("Working at {}", i);
            state[i-1] = state[i];
        }
        // zeros have become 6s, and have made 8s
        state[6] += zeros;
        state[8] = zeros;

        // println!("Method B day {}, state: {:?}, count {}", day, state,
        //      state.iter().sum::<u64>());
    }

    state.iter().sum()
}

fn part_a(days: u64) -> u64 {
    let raw = fs::read_to_string("../inputs/6.txt").unwrap();
    // let raw = String::from("3,4,3,1,2");

    let mut state: Vec<u64> = raw.split(",").map(|x| x.trim().parse::<u64>().unwrap()).collect();

    fn evolve(s: u64) -> u64 {
        if s > 0 {
            s - 1
        } else {
            6
        }
    }

    for i in 1..=days {
        let children = state.clone().into_iter().filter(|x| *x == 0).count();
        state = state.into_iter().map(&evolve).collect();
        for i in 0..children {
            state.push(8);
        }
        println!("After {} days: {:?}, count {}", i,
                 make_state(&state), state.len());
    }
    state.len() as u64
}

fn main() {
    println!("Part 1 (method A): {}", part_a(80));
    println!("Part 1 (method B): {}", part_b(80));
    println!("Part 1 (method B): {}", part_b(256));
    // println!("Part 2 (method B): {}", part_b(256));
    // let mut v = vec![1,2,3];
    // v = v.into_iter().map(|x| x+1).collect();
    // println!("{:?}", v);
}
