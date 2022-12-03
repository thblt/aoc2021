use lib::*;

/// That's how we cheat the "infinite image" rule.
///
///  1. We load the image with extra pixels on each margin, more than
///     there are steps.
///
///  2. When we need to read a pixel outside the image boundaries, we
///     fall back to the pixel at (0,0), which is the "background",
///     which alternates at each iteration step, because algorithm[0]
///     == true and algorith[511] == false; so each iteration toggles
///     the background color.
fn get_or_tl(image: &Vec2D<bool>, x: isize, y: isize, ret: usize) -> usize {
    if match image.safe_index(x, y) {
        Some(x) => x,
        None => image[(0, 0)],
    } {
        ret
    } else {
        0
    }
}

fn enhance(alg: &Vec<bool>, image: &Vec2D<bool>) -> Vec2D<bool> {
    let mut ret = image.clone();
    let mut index;
    for x in 0..image.width() as isize {
        for y in 0..image.height() as isize {
            index = get_or_tl(&image, x - 1, y - 1, 256)
                + get_or_tl(&image, x, y - 1, 128)
                + get_or_tl(&image, x + 1, y - 1, 64)
                + get_or_tl(&image, x - 1, y, 32)
                + get_or_tl(&image, x, y, 16)
                + get_or_tl(&image, x + 1, y, 8)
                + get_or_tl(&image, x - 1, y + 1, 4)
                + get_or_tl(&image, x, y + 1, 2)
                + get_or_tl(&image, x + 1, y + 1, 1);
            ret[(x, y)] = alg[index];
        }
    }
    ret
}

fn draw(image: &Vec2D<bool>) {
    for y in 0..image.height() as isize {
        for x in 0..image.width() as isize {
            print!("{}", if image[(x, y)] { "#" } else { "." })
        }
        println!();
    }
}

fn read_input(s: &str, side: usize, extra_pixels: usize) -> (Vec<bool>, Vec2D<bool>) {
    let mut input = read_lines(s).unwrap();
    let alg = input
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();

    let mut image = Vec2D::<bool>::new(side + extra_pixels * 2, side + extra_pixels * 2, false);
    println!("{},{}", image.height(), image.width());

    let mut y = extra_pixels as isize;
    let mut x;
    for line in input.skip(1) {
        let line = line.unwrap();
        // println!("{} {}", y, &line.len());
        x = extra_pixels as isize;
        for c in line.chars() {
            image[(x, y)] = if c == '#' { true } else { false };
            x += 1;
        }
        println!();
        y += 1;
    }

    (alg, image)
}

fn main() {
    let (alg, mut image) = read_input("../inputs/20.txt", 100, 60);
    print!("Stepped enhancement…");

    for i in 1..=50 {
        image = enhance(&alg, &image);
        print!(" {}", i);
    }
    println!();

    // draw(&image);
    let mut count = 0;
    for x in 10..(image.width() - 10) as isize {
        for y in 10..(image.width() - 10) as isize {
            if image[(x, y)] {
                count += 1;
            }
        }
    }
    println!("Grand total: {}", count);
}
