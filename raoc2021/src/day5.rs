use lib::*;
use std::cmp::{max, min};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

// A stupid 2D vector
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
}

impl<T: Copy> Vec2D<T> {
    fn new(width: usize, height: usize, value: T) -> Vec2D<T> {
        let mut vec = Vec::with_capacity(height * width);

        for i in 0..width * height {
            vec.push(value);
        }

        println!("Vec size is {}", vec.len());

        Vec2D { vec, width, height }
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

impl Vec2D<u32> {
    fn draw(&self) {
        for i in 0..self.vec.capacity() {
            if i % self.width == 0 {
                println!();
            }
            print!(
                "{}",
                match self.vec[i] {
                    0 => '.',
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    _ => '+',
                }
            );
        }
        println!();
    }
}

use regex::Regex;
#[derive(Copy, Clone, Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
/// Parse a line description of the form
/// 12,29 -> 28,12
/// Into a Line object.
impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let caps = re.captures(s).unwrap();
        // println!("{} parses as {:?}", s, caps);
        let x1 = caps[1].parse::<usize>().unwrap();
        let y1 = caps[2].parse::<usize>().unwrap();
        let x2 = caps[3].parse::<usize>().unwrap();
        let y2 = caps[4].parse::<usize>().unwrap();
        Ok(Line { x1, y1, x2, y2 })
    }
}

impl Line {
    /// t if line is horizontal or vertical
    fn is_horz_or_vert(&self) -> bool {
        (self.x1 == self.x2) || (self.y1 == self.y2)
    }
}

fn run() -> u32 {
    let lines: Vec<Line> = read_lines("../inputs/5.txt")
        .unwrap()
        .map(|l| l.unwrap().parse::<Line>().unwrap())
        //        .filter(&Line::is_horz_or_vert)
        .collect();

    fn maxfld((x_max, y_max): (usize, usize), Line { x1, y1, x2, y2 }: Line) -> (usize, usize) {
        (max(x_max, max(x1, x2)), (max(y_max, max(y1, y2))))
    }

    let (x_max, y_max) = lines.clone().into_iter().fold((0, 0), maxfld);

    let mut space: Vec2D<u32> = Vec2D::new(x_max + 1, y_max + 1, 0);

    for l in lines {
        if l.x1 == l.x2 || l.y1 == l.y2 {
            let x1 = min(l.x1, l.x2);
            let x2 = max(l.x1, l.x2);
            let y1 = min(l.y1, l.y2);
            let y2 = max(l.y1, l.y2);
            // Straight line
            for x in x1..=x2 {
                for y in y1..=y2 {
                    space[(x, y)] += 1;
                }
            }
        } else {
            let x1 = l.x1 as isize;
            let x2 = l.x2 as isize;
            let y1 = l.y1 as isize;
            let y2 = l.y2 as isize;
            let xdir: isize = if x1 > x2 { -1 } else { 1 };
            let ydir: isize = if y1 > y2 { -1 } else { 1 };
            let dist = max(x1, x2) - min(x1, x2);
            // Diagonal line
            // println!("Drawing a diagonal line: {},{} -> {},{}", x1, y1, x2, y2);
            for i in 0..=dist {
                let x: usize = (x1 + (xdir * i)) as usize;
                let y: usize = (y1 + (ydir * i)) as usize;
                // println!("Drawing at {},{}", x,y);
                space[(x, y)] += 1;
            }
        }
    }

    fn count_fld(acc: u32, item: &u32) -> u32 {
        if *item > 1 {
            acc + 1
        } else {
            acc
        }
    }
    space.draw();
    space.vec.iter().fold(0, &count_fld)
}

fn main() {
    println!("Part 2: {}", run());
    println!("Part 1 is just part 2 with the filter() line in run() uncommented.");
}
