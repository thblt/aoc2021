use lib::*;

const MASK_FLASHED: u8 = 0b1000000;
const MASK_DEFLASH: u8 = !MASK_FLASHED;

/// Handle flashes.
fn flash(map: &mut Vec2D<u8>) -> u32 {
    let mut flashes = 0;
    loop {
        let mut again = false;
        for x in 0..map.width() as isize {
            for y in 0..map.height() as isize {
                if map[(x, y)] > 9 && (map[(x, y)] & MASK_FLASHED == 0) {
                    flashes += 1;
                    again = true;
                    for x2 in [-1, 0, 1] {
                        for y2 in [-1, 0, 1] {
                            if map.test_coords(x + x2, y + y2) {
                                map[(x + x2, y + y2)] += 1;
                            }
                        }
                    }
                    map[(x, y)] = MASK_FLASHED
                }
            }
        }
        if !again {
            break;
        }
    }
    // "Finally, any octopus that flashed during this step has its energy level set to 0"
    for o in &mut map.vec {
        if *o > 10 {
            *o = 0;
        }
    }
    if flashes as usize == map.width() * map.height() {
        println!("All flashed!");
    }
    flashes
}

fn step(map: &mut Vec2D<u8>) -> u32 {
    for value in &mut map.vec {
        *value += 1;
    }
    flash(map)
}

fn main() {
    let raw: Vec<String> = read_lines("../inputs/11.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    let height = raw.len();
    let width = raw[1].len();

    let mut map: Vec2D<u8> = Vec2D::new(width, height, 0);

    for y in 0..height {
        for x in 0..width {
            map[(x as isize, y as isize)] = read_digit(raw[y].chars().nth(x).unwrap());
        }
    }

    let mut map_b = map.clone();

    let mut flashes = 0;
    let total = (width * height) as u32;

    for _ in 0..100 {
        flashes += step(&mut map);
    }
    println!("Part 1: there were {} flashes", flashes);

    let mut i = 0;
    loop {
        i += 1;
        if total == step(&mut map_b) {
            println!("All flashed at step {}", i);
            break;
        }
    }
}
