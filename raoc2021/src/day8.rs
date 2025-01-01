use lib::*;

// Segments masks
const MASK_A: u8 = 1;
const MASK_B: u8 = 2;
const MASK_C: u8 = 4;
const MASK_D: u8 = 8;
const MASK_E: u8 = 16;
const MASK_F: u8 = 32;
const MASK_G: u8 = 64;

// Digit representations.
const REPR_0: u8 = MASK_A | MASK_B | MASK_C | MASK_E |  MASK_F | MASK_G;
const REPR_1: u8 = MASK_C | MASK_F;
const REPR_2: u8 = MASK_A | MASK_C | MASK_D | MASK_E | MASK_G;
const REPR_3: u8 = MASK_A | MASK_C | MASK_D | MASK_F | MASK_G;
const REPR_4: u8 = MASK_B | MASK_C | MASK_D | MASK_F;
const REPR_5: u8 = MASK_A | MASK_B | MASK_D | MASK_F | MASK_G;
const REPR_6: u8 = MASK_A | MASK_B | MASK_D | MASK_E | MASK_F | MASK_G;
const REPR_7: u8 = MASK_A | MASK_C | MASK_F;
const REPR_8: u8 = MASK_A | MASK_B | MASK_C | MASK_D | MASK_E | MASK_F | MASK_G;
const REPR_9: u8 = MASK_A | MASK_B | MASK_C | MASK_D | MASK_F | MASK_G;

fn count_bits(n: &u8) -> u8 {
    let mut ret = 0;
    if 0 != n & 1 {
        ret += 1
    };
    if 0 != n & 2 {
        ret += 1
    };
    if 0 != n & 4 {
        ret += 1
    };
    if 0 != n & 8 {
        ret += 1
    };
    if 0 != n & 16 {
        ret += 1
    };
    if 0 != n & 32 {
        ret += 1
    };
    if 0 != n & 64 {
        ret += 1
    };
    if 0 != n & 128 {
        ret += 1
    };
    ret
}

/// Read a number as a segment mask and return its value.
fn read_segments(segments: u8) -> u8 {
    match segments {
        REPR_0 => 0,
        REPR_1 => 1,
        REPR_2 => 2,
        REPR_3 => 3,
        REPR_4 => 4,
        REPR_5 => 5,
        REPR_6 => 6,
        REPR_7 => 7,
        REPR_8 => 8,
        REPR_9 => 9,
        _ => panic!("Bad segment combination")
    }
}

fn solve(patterns: &[u8], puzzle: &[u8]) -> u32 {
    // First step. We use patterns to associate lines to segments.

    // Seven variables mapping segment X (seg_X) to candidate LINES.
    // Base value is b01111111 = 127 = all segments are equally likely.
    let mut seg_a = 127;
    let mut seg_b = 127;
    let mut seg_c = 127;
    let mut seg_d = 127;
    let mut seg_e = 127;
    let mut seg_f = 127;
    let mut seg_g = 127;

    let mut fives: Vec<u8> = vec![];
    let mut sixes: Vec<u8> = vec![];

    // Return a mask of bit differences in three bytes.
    fn diff(a: &u8, b: &u8, c: &u8) -> u8 {
        (a^b) | (a^c) | (b^c)
    }

    for item in patterns {
        match count_bits(item) {
            2 => {
                // This is digit 1.
                seg_c = seg_c & item;
                seg_f = seg_f & item;
            }
            3 => {
                // This is digit 7
                seg_a = seg_a & item;
                seg_c = seg_c & item;
                seg_f = seg_f & item;
            }
            4 => {
                // This is digit 4
                seg_b = seg_b & item;
                seg_c = seg_c & item;
                seg_d = seg_d & item;
                seg_f = seg_f & item;
            }
            5 => {
                // Either 2, 3 or 5.
                fives.push(*item);
            }
            6 => {
                // Either 0, 6 or 9
                sixes.push(*item);
            }
            7 => {} // This is 8, we don't care.
            _ => {
                // Abnormal
                panic!("Nope")
            }
        }
    }

    let diff5 = diff(&fives[0], &fives[1], &fives[2]);
    seg_b = seg_b & diff5;
    seg_e = seg_e & diff5;
    seg_f = seg_f & diff5;

    let inter5 = fives[0] & fives[1] & fives[2];
    seg_a = seg_a & inter5;
    seg_d = seg_d & inter5;
    seg_g = seg_g & inter5;

    let diff6 = diff(&sixes[0], &sixes[1], &sixes[2]);
    seg_c = seg_c & diff6;
    seg_d = seg_d & diff6;
    seg_e = seg_e & diff6;

    // Finally…  Not too much reasoning there: just ran the code in a
    // loop, looked for intersections between solved segments and
    // unsolved ones, and applied substractions.
    seg_e = seg_e & !seg_c;
    seg_f = seg_f & !seg_c;
    seg_b = seg_b & !seg_f;
    seg_b = seg_b & !seg_c;
    seg_g = seg_g & !seg_a;
    seg_g = seg_g & !seg_d;

    // Part 2: solve the puzzle

    let mut total: u32 = 0;
    for piece in puzzle {
        total *= 10;
        let mut pattern = 0;
        if piece & seg_a != 0 { pattern |= MASK_A }
        if piece & seg_b != 0 { pattern |= MASK_B }
        if piece & seg_c != 0 { pattern |= MASK_C }
        if piece & seg_d != 0 { pattern |= MASK_D }
        if piece & seg_e != 0 { pattern |= MASK_E }
        if piece & seg_f != 0 { pattern |= MASK_F }
        if piece & seg_g != 0 { pattern |= MASK_G }

        // print!("{}", )
        total += read_segments(pattern) as u32;
    }
    println!("{}",total);
    total
}

fn char_to_mask(c: char) -> u8 {
    match c {
        'a' => MASK_A,
        'b' => MASK_B,
        'c' => MASK_C,
        'd' => MASK_D,
        'e' => MASK_E,
        'f' => MASK_F,
        'g' => MASK_G,
        _ => panic!("Bad input"),
    }
}


fn print_mask(m: u8) {
    fn c(cond: u8, s: &str) -> String {
        if cond == 0 { " ".to_string() } else { s.to_string() }
    }

    println!("   a \n  {} \nb{}   {}c\n d{} \ne{}   {}f\n g{}",
             c(m&MASK_A, "━━━"),
             c(m&MASK_B, "┃"),
             c(m&MASK_C, "┃"),
             c(m&MASK_D, "━━━"),
             c(m&MASK_E, "┃"),
             c(m&MASK_F, "┃"),
             c(m&MASK_G, "━━━"));
}

fn parse(s: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![];

    for item in s.split(' ') {
        let mut val: u8 = 0;
        for c in item.chars() {
            val |= char_to_mask(c)
        }
        ret.push(val);
    }
    ret
    }

    fn main() {
        let input = read_lines("../inputs/8.txt").unwrap();

        let mut count = 0;
        let mut sum = 0;
        for line in input {
            let parts: Vec<String> = line.unwrap().split(" | ").map(String::from).collect();

            let patterns = parse(&parts[0]);
            let puzzle = parse(&parts[1]);
            sum += solve(&patterns, &puzzle);
            count += parts[1]
                .split(" ")
                .filter(|item| [2, 3, 4, 7].contains(&item.len()))
                .count();
        }

        println!("Part A: {}\nPart B: {}", count, sum)
    }
