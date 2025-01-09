type Ocean = Vec<Vec<Tile>>;
#[derive(PartialEq, Clone, Copy)]
enum Tile {
    East,
    South,
    Nothing,
}

fn step(ocean: &mut Ocean, dir: (usize, usize), obj: Tile) -> bool {
    let width = ocean[0].len();
    let height = ocean.len();

    let mut cucumbers = vec![];
    let mut nothings = vec![];
    for y in 0..height {
        for x in 0..width {
            if ocean[y][x] == obj {
                let next_x = if x + dir.0 == width { 0 } else { x + dir.0 };
                let next_y = if y + dir.1 == height { 0 } else { y + dir.1 };
                if ocean[next_y][next_x] == Tile::Nothing {
                    nothings.push((x, y));
                    cucumbers.push((next_x, next_y));
                }
            }
        }
    }
    if cucumbers.is_empty() {
        return false;
    }

    for (mut list, object) in [(cucumbers, obj), (nothings, Tile::Nothing)] {
        for (x, y) in list.drain(..) {
            ocean[y][x] = object;
        }
    }

    true
}

fn main() {
    let mut ocean: Vec<Vec<Tile>> = vec![];
    for line in std::fs::read_to_string("../inputs/25.txt")
        .unwrap()
        .split("\n")
    {
        if line.is_empty() {
            continue;
        }
        let mut out_line = vec![];
        for elem in line.chars() {
            out_line.push(match elem {
                'v' => Tile::South,
                '>' => Tile::East,
                '.' => Tile::Nothing,
                _ => panic!(),
            });
        }
        ocean.push(out_line);
    }

    for part1 in 1.. {
        let east = step(&mut ocean, (1, 0), Tile::East);
        let south = step(&mut ocean, (0, 1), Tile::South);
        // Remember you can't do: if step(east) || step (north)
        // because || is short-circuiting: you will miss north steps.
        if !(east || south) {
            println!("Part 1: {part1}");
            break;
        }
    }
}
