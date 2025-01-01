use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{Index, IndexMut};
use std::path::Path;

#[derive(Clone)]
pub struct Vec2D<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2D<T> {
    pub fn to_index(&self, (x, y): (isize, isize)) -> usize {
        let x = x as usize;
        let y = y as usize;

        y * self.width + x
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_coords(&self, idx: usize) -> (isize, isize) {
        let x = (idx % self.width) as isize;
        let y = (idx / self.width) as isize;
        (x, y)
    }

    pub fn test_coords(&self, x: isize, y: isize) -> bool {
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
    pub fn new(width: usize, height: usize, value: T) -> Vec2D<T> {
        let mut vec = Vec::with_capacity(height * width);

        for _ in 0..width * height {
            vec.push(value);
        }

        Vec2D { vec, width, height }
    }

    pub fn safe_index(&self, x: isize, y: isize) -> Option<T> {
        if self.test_coords(x, y) {
            Some(self[(x, y)])
        } else {
            None
        }
    }

    pub fn draw_with(&self, func: &dyn Fn(&T) -> String) {
        for i in 0..self.vec.len() {
            if i % self.width == 0 {
                println!();
            }
            let val = self.vec[i];
            print!("{}", func(&val));
        }
        println!();
    }
}

impl<T> Index<(isize, isize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, coords: (isize, isize)) -> &Self::Output {
        &self.vec[self.to_index(coords)]
    }
}

impl<T> IndexMut<(isize, isize)> for Vec2D<T> {
    fn index_mut(&mut self, coords: (isize, isize)) -> &mut Self::Output {
        let i = self.to_index(coords);
        &mut self.vec[i]
    }
}

impl Vec2D<u8> {
    pub fn draw(&self) {
        for i in 0..self.vec.capacity() {
            if i % self.width == 0 {
                println!();
            }
            let val = self.vec[i];
            print!(
                "{}",
                if val < 10 {
                    val.to_string()
                } else {
                    String::from("X")
                }
            );
        }
        println!();
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn bools_to_bin_string(n: &[bool]) -> String {
    n.iter().map(|x| if *x { '1' } else { '0' }).collect()
}

pub fn read_digit(c: char) -> u8 {
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
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!("Bad input"),
    }
}

use std::ops::Sub;
pub fn abs_diff<T: Copy + Ord + Sub>(a: T, b: T) -> <T as Sub>::Output {
    use std::cmp::{max, min};
    max(a, b) - min(a, b)
}
