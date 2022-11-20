// Looking for an optimal path.
//
// Naive approach: systematic enumeration (this is the only way this
// time) *but* we try the shortest path first, and abandon all paths
// with a higher risk level.
//
// It means that at each point, we move from the direction that will
// bring us the closest to target, to the direction that will bring up
// farthest.

use lib::*;
use std::cmp::Ord;

/// Distance between two points, as the sum of the absolute difference
/// of each coordinate.
fn dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    abs_diff(x1, x2) + abs_diff(y1, y2)
}

struct Walker {
    best_risk: Option<u32>,
}

fn walk(
    maze: &Vec2D<u8>,
    from: (isize, isize),
    to: (isize, isize),
    path: &mut Vec<(isize,isize)>,
    risk: u32,
    walker: &mut Walker,
) {
    let fx = from.0;
    let fy = from.1;
    let tx = to.0;
    let ty = to.1;

    // If our total risk is greater than the best risk, this is a dead
    // end.  We abort.
    if let Some(best_risk) = walker.best_risk {
        if best_risk < risk {
            // println!("Excessive risk {} (best is {}), turning back.", risk, best_risk);
            return;
        }
    }

    // If we've arrived, we've set a new record for best risk.
    if (fx == tx) && (fy == ty) {
        if let Some(best_risk) = walker.best_risk {
            if risk >= best_risk {
                return
            }
        }
        println!("Reached! New best is {}, through:", risk);
        let mut mz = maze.clone();
        for point in path {
            mz[*point] = 10;
        }
        mz[(tx,ty)] = 10;
        mz.draw();
        walker.best_risk = Some(risk);
        return;
    }

    let moves = [(fx - 1, fy), (fx + 1, fy), (fx, fy - 1), (fx, fy + 1)];
    let mut candidates = moves
        .iter()
        .filter(|cand| maze.test_coords(cand.0, cand.1))
        .filter(|cand| !path.contains(cand))
        .collect::<Vec<&(isize, isize)>>();

    candidates.sort_by(|a, b| isize::cmp(&dist(a.0, a.1, tx, ty), &dist(b.0, b.1, tx, ty)));

    path.push((fx,fy));

    // for c in &candidates {
    //     print!("[{},{} distance={}]", c.0, c.1, dist(c.0, c.1, tx, ty));
    // }
    // println!();

    for (cx, cy) in candidates {
        let risk_extra = maze[(*cx, *cy)] as u32;
        // println!("At {},{}, moving to {},{}.", fx, fy, cx, cy);
        walk(maze, (*cx, *cy), (tx, ty), &mut path.clone(), risk + risk_extra, walker);
    }
}

fn main() {
    let maze = read_input();
    maze.draw();
    let dest = (maze.width() as isize - 1, maze.height() as isize - 1);
    let mut walker = Walker { best_risk: None };

    // BUG:
    // At 9, 1, next steps: [(8, 1), (9, 2), (9, 0)]
    println!("{}", dist(8,1,9,9));
    println!("{}", dist(9,2,9,9));

    println!("Trying to reach {},{}", &dest.0, &dest.1);

    let mut path: Vec<(isize,isize)> = vec![];
    walk(&maze, (0, 0), dest, &mut path, 0, &mut walker);

    // println!("Part 1: {}", walker.best_risk.unwrap());
}

fn read_input() -> Vec2D<u8> {
    let lines: Vec<String> = read_lines("../inputs/15.txt")
        .unwrap()
        .map(&std::result::Result::unwrap)
        .collect();
    let mut ret = Vec2D::new(lines[0].len(), lines.len(), 0);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            ret[(x as isize, y as isize)] = lines[y]
                .chars()
                .nth(x)
                .unwrap()
                .to_string()
                .parse::<u8>()
                .unwrap();
        }
    }
    ret
}
