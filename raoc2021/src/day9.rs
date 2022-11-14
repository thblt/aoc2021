use lib::*;
// use std::cmp::{max, min};
use std::ops::{Index, IndexMut};

struct Vec2D<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2D<T> {
    fn to_index(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn to_coords(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn test_coords(&self, x: isize, y: isize) -> bool {
        if x < 0 {
            return false;
        }
        if y < 0 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;

        if x >= self.width {
            return false;
        }
        if y >= self.height {
            return false;
        }
        true
    }
}

impl<T: Copy> Vec2D<T> {
    fn new(width: usize, height: usize, value: T) -> Vec2D<T> {
        let mut vec = Vec::with_capacity(height * width);

        for _ in 0..width * height {
            vec.push(value);
        }

        println!("Vec size is {}", vec.len());

        Vec2D { vec, width, height }
    }

    fn safe_index(&self, x: isize, y: isize) -> Option<T> {
        if self.test_coords(x, y) {
            Some(self[(x as usize, y as usize)])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        &self.vec[self.to_index(coords)]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let i = self.to_index(coords);
        &mut self.vec[i]
    }
}

impl Vec2D<u8> {
    fn draw(&self) {
        for i in 0..self.vec.capacity() {
            if i % self.width == 0 {
                println!("");
            }
            let val = self.vec[i];
            print!("{}", if val < 10 { val.to_string() } else { String::from("X") });
        }
        println!("");
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let val = self[(x, y)];
        let x = x as isize;
        let y = y as isize;

        let u = self.safe_index(x, y - 1);
        let d = self.safe_index(x, y + 1);
        let l = self.safe_index(x - 1, y);
        let r = self.safe_index(x + 1, y);
        let mut ret = true;
        for side in &[u, d, l, r] {
            if let Some(side) = side {
                ret &= side > &val
            }
        }
        ret
    }
}

fn read_number(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Bad input"),
    }
}

fn explore_basin(map: &mut Vec2D<u8>, x: usize, y: usize, size: u32) -> u32 {
    // To explore a basin, we recursively look for higher, but < 9,
    // points around our starting point.  Because we don't want to
    // explore the same point twice, we mark our location by or-ing it
    // with 0b10000000.
    let mark = 128;
    let value = map[(x, y)];
    let mut ret = size;

    if value & mark != 0 {
        return 0; // We're been there already.
    } else {
        ret += 1;
        map[(x, y)] |= mark; // Mark our place.
        println!("Point at {},{}, value is {}, size is now {}", x, y,
                 value, size);
        // map.draw();
    }

    // Compute corners
    let xi = x as isize;
    let yi = y as isize;

    for step in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x2 = xi + step.0;
        let y2 = yi + step.1;
        if !map.test_coords(x2, y2) {
            continue;
        }
        if let Some(next_value) = map.safe_index(x2, y2) {
            if next_value < 9 && next_value > value {
                ret += explore_basin(map, x2 as usize, y2 as usize, 0);
            }
        }
    }
    ret
}

fn main() {
    let raw: Vec<String> = read_lines("../inputs/9.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    let height = raw.len();
    let width = raw[1].len();

    let mut map: Vec2D<u8> = Vec2D::new(width, height, 0);

    for y in 0..height {
        for x in 0..width {
            map[(x, y)] = read_number(raw[y].chars().nth(x).unwrap());
        }
    }

    let mut low_points: Vec<(usize, usize)> = vec![];
    let mut total: u32 = 0;
    for y in 0..height {
        for x in 0..width {
            if map.is_low_point(x, y) {
                low_points.push((x, y));
                total += 1;
                total += map[(x, y)] as u32;
            }
        }
    }

    let mut basins: Vec<u32> = vec![];
    for b in low_points {
        println!("Exploring: {:?}", b);
        basins.push(explore_basin(&mut map, b.0, b.1, 0));

        println!("Cleaning…");
        for i in 0..map.vec.len() {
            map.vec[i] &= 0b01111111
        }
        map.draw();
    }

    println!("Part 1: {}", total);

    basins.sort();
    println!("Basins {:?}", basins);
    println!("Part 2: {:?}", basins.pop().unwrap()*basins.pop().unwrap()*basins.pop().unwrap());

    // To find basins, we iterate over the list of low points.
}
