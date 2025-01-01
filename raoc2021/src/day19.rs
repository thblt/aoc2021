/* This is excessively brute-force, but it works.
 *
 * Part 1 could really use some optimizations.
 *
 * For part 2, we simply insert another beacon at 0,0,0 relative to
 * each scanner, with a special tag (that's the `scanner` field in
 * `Coord`) to identify them.  At the end of part 1, these
 * pseudo-beacons will have been translated to the position of each
 * other scanner, relative to scanner 0.
 *
 * For part 2, we then collect back those pseudo-beacons (identified
 *  by their tag and compute their Manhattan distance.
 */

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use lib::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Ord, PartialOrd)]
enum Axis {
    X,
    Y,
    Z,
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Axis::X => "X",
                Axis::Y => "Y",
                Axis::Z => "Z",
            }
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Ord, PartialOrd)]
struct CoordSystem {
    axis_1: Axis,
    axis_1_sign: bool,
    axis_2: Axis,
    axis_2_sign: bool,
    axis_3: Axis,
    axis_3_sign: bool,
}

impl CoordSystem {
    /// Return a new coordinate system.
    fn new(
        axis_1: Axis,
        axis_1_sign: bool,
        axis_2: Axis,
        axis_2_sign: bool,
        axis_3: Axis,
        axis_3_sign: bool,
    ) -> CoordSystem {
        if (axis_1 == axis_2) || (axis_2 == axis_3) || (axis_1 == axis_3) {
            panic!("Duplicate axis");
        }
        CoordSystem {
            axis_1,
            axis_1_sign,
            axis_2,
            axis_2_sign,
            axis_3,
            axis_3_sign,
        }
    }

    /// Rotate this coordinate system by 90° along the ABSOLUTE axis
    /// `axis`.
    fn rotate(&mut self, along: Axis, forward: bool) {
        use Axis::*;
        let axis: Axis;
        let sign: bool;
        match along {
            X => {
                axis = self.axis_2;
                sign = self.axis_2_sign;
                if forward {
                    self.axis_2 = self.axis_3;
                    self.axis_2_sign = self.axis_3_sign;
                    self.axis_3 = axis;
                    self.axis_3_sign = !sign;
                } else {
                    self.axis_2 = self.axis_3;
                    self.axis_2_sign = !self.axis_3_sign;
                    self.axis_3 = axis;
                    self.axis_3_sign = sign;
                }
            }
            Y => {
                axis = self.axis_1;
                sign = self.axis_1_sign;
                if forward {
                    self.axis_1 = self.axis_3;
                    self.axis_1_sign = !self.axis_3_sign;
                    self.axis_3 = axis;
                    self.axis_3_sign = sign;
                } else {
                    self.axis_1 = self.axis_3;
                    self.axis_1_sign = self.axis_3_sign;
                    self.axis_3 = axis;
                    self.axis_3_sign = !sign;
                }
            }
            Z => {
                axis = self.axis_2;
                sign = self.axis_2_sign;
                if forward {
                    self.axis_2 = self.axis_1;
                    self.axis_2_sign = !self.axis_1_sign;
                    self.axis_1 = axis;
                    self.axis_1_sign = sign;
                } else {
                    self.axis_2 = self.axis_1;
                    self.axis_2_sign = self.axis_1_sign;
                    self.axis_1 = axis;
                    self.axis_1_sign = !sign;
                }
            }
        }
    }

    fn get_axis_sign(&self, axis: Axis) -> bool {
        if self.axis_1 == axis {
            self.axis_1_sign
        } else if self.axis_2 == axis {
            self.axis_2_sign
        } else if self.axis_3 == axis {
            self.axis_3_sign
        } else {
            panic!("Abnormal system");
        }
    }

    fn all() -> Vec<Self> {
        use Axis::*;
        let mut cs = CoordSystem::default();

        let mut ret: Vec<CoordSystem> = vec![];

        // Outer loop: we bring each side up in turn
        for side_up in [(1, X), (1, X), (1, X), (1, X), (1, Z), (2, X)] {
            for _ in 0..side_up.0 {
                cs.rotate(side_up.1, true);
            }
            // Inner loop: we rotate around the up side.
            for _ in 0..4 {
                cs.rotate(Y, true);
                ret.push(cs);
            }
        }
        ret
    }
}

impl Display for CoordSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{},{}{},{}{}]",
            if self.axis_1_sign { "+" } else { "-" },
            self.axis_1,
            if self.axis_2_sign { "+" } else { "-" },
            self.axis_2,
            if self.axis_3_sign { "+" } else { "-" },
            self.axis_3,
        )
    }
}

impl Default for CoordSystem {
    fn default() -> Self {
        use Axis::*;
        CoordSystem::new(X, true, Y, true, Z, true)
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug, Hash)]
struct Coord {
    a: i32,
    b: i32,
    c: i32,
    system: CoordSystem,
    scanner: bool,
}

impl Coord {
    fn new(a: i32, b: i32, c: i32) -> Coord {
        Coord {
            a,
            b,
            c,
            system: CoordSystem::default(),
            scanner: false,
        }
    }

    fn get_axis(&self, axis: Axis) -> i32 {
        if self.system.axis_1 == axis {
            self.a
        } else if self.system.axis_2 == axis {
            self.b
        } else if self.system.axis_3 == axis {
            self.c
        } else {
            panic!("Abnormal system");
        }
    }

    fn set_axis(&mut self, axis: Axis, value: i32) {
        if self.system.axis_1 == axis {
            self.a = value
        } else if self.system.axis_2 == axis {
            self.b = value
        } else if self.system.axis_3 == axis {
            self.c = value
        }
    }

    fn translate(&self, to: CoordSystem) -> Coord {
        use Axis::*;
        let minus_x = self.system.get_axis_sign(X) != to.get_axis_sign(X);
        let minus_y = self.system.get_axis_sign(Y) != to.get_axis_sign(Y);
        let minus_z = self.system.get_axis_sign(Z) != to.get_axis_sign(Z);
        let x = self.get_axis(X);
        let y = self.get_axis(Y);
        let z = self.get_axis(Z);
        let mut ret = Coord {
            a: 0,
            b: 0,
            c: 0,
            system: to,
            scanner: self.scanner,
        };
        ret.set_axis(X, if minus_x { -x } else { x });
        ret.set_axis(Y, if minus_y { -y } else { y });
        ret.set_axis(Z, if minus_z { -z } else { z });
        ret
    }

    /// Compute the required translation, on all three axes, to move
    /// from this Coord to another.
    fn sub(&self, other: &Coord) -> (i32, i32, i32) {
        (other.a - self.a, other.b - self.b, other.c - self.c)
    }

    /// Apply a computed translation, on all three axes, to move
    /// this Coord to another.
    fn add(&mut self, (ta, tb, tc): (i32, i32, i32)) {
        self.a -= ta;
        self.b -= tb;
        self.c -= tc;
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{}@{})", self.a, self.b, self.c, self.system)
    }
}

fn read_input(file: &str) -> Vec<HashSet<Coord>> {
    let mut ret: Vec<HashSet<Coord>> = vec![];
    let mut start = true;
    let template: HashSet<Coord> = HashSet::from_iter([Coord {
        a: 0,
        b: 0,
        c: 0,
        system: CoordSystem::default(),
        scanner: true,
    }]);
    let mut current = template.clone();

    for line in read_lines(file).unwrap() {
        let line = line.unwrap();
        if start {
            // Skip first line
            start = false
        } else if line.is_empty() {
            start = true;
            ret.push(current);
            current = template.clone();
        } else {
            let parts: Vec<i32> = line.split(",").map(|l| l.parse::<i32>().unwrap()).collect();
            current.insert(Coord {
                a: parts[0],
                b: parts[1],
                c: parts[2],
                system: CoordSystem::default(),
                scanner: false,
            });
        }
    }
    ret.push(current);
    ret
}

fn pair_distances(cs: &HashSet<Coord>) -> HashMap<(i32, i32, i32), (Coord, Coord)> {
    let mut ret: HashMap<(i32, i32, i32), (Coord, Coord)> = HashMap::new();
    for a in cs {
        for b in cs {
            if a > b {
                ret.insert(a.sub(b), (*a, *b));
            }
        }
    }
    ret
}

/// Merge FROM into TO, draining FROM, adding TRANS first.
fn merge(to: &mut HashSet<Coord>, from: &HashSet<Coord>, trans: (i32, i32, i32), cs: CoordSystem) {
    // println!("MERGE init sum {}+{}={}", from.len(), to.len(), from.len() + to.len());
    for c in from {
        let mut c = *c;
        c.add(trans);
        c.system = cs;
        to.insert(c);
    }
}

/// Compute the intersection of two vectors.
fn vec_inter<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Copy + Eq + PartialOrd + Ord,
{
    let mut ret: Vec<T> = vec![];
    a.sort();
    b.sort();
    let mut ia = 0;
    let mut ib = 0;
    while ia < a.len() && ib < b.len() {
        use std::cmp::Ordering::*;
        match a[ia].cmp(&b[ib]) {
            Equal => {
                ret.push(a[ia]);
                ia += 1;
                ib += 1;
            }
            Less => {
                ia += 1;
            }
            Greater => {
                ib += 1;
            }
        }
    }

    ret
}

fn main() {
    // The goal is to merge objects from all scanners into scanner 0.
    //
    //  - Traverse all possible scanner pairs until we find a pair
    //    (A,B) with at least twelve overlapping objects.
    //
    //  - Translate coordinates in B to A (that is, take any
    //    overlapping object, substract its coordinates in A to those
    //    in B, add the result to all coordinates of objects in B) and
    //    merge the two lists in one, A, removing duplicates.  Clear
    //    list B and remove it.
    //
    //  - Restart at beginning until there's only one list remaining.
    let mut input = read_input("../inputs/19.txt");
    let mut objects: HashSet<Coord> = HashSet::new();
    let mut scanner_dists: Vec<Coord> = vec![];

    let mut keep_going = true;
    println!("There are {} scanners.", input.len());
    println!(
        "Default orientation: {}",
        input[0].iter().next().unwrap().system
    );
    while keep_going {
        keep_going = false;
        // Outer loop: first traversal of scanners.
        for i in 0..input.len() {
            let dists_0 = pair_distances(&input[i]);
            let dists_0_only: Vec<(i32, i32, i32)> = dists_0.keys().copied().collect();
            // Inner loop.  Other scanner.
            for j in i + 1..input.len() {
                for cs in CoordSystem::all() {
                    let all_objects = input[j]
                        .iter()
                        .map(|c| c.translate(cs))
                        .collect::<HashSet<Coord>>();
                    let dists = pair_distances(&all_objects);
                    let intersection: Vec<(i32, i32, i32)> = vec_inter(
                        dists.keys().copied().collect::<Vec<(i32, i32, i32)>>(),
                        dists_0_only.clone(),
                    );

                    objects.clear();
                    for p in &intersection {
                        objects.insert(dists_0[p].0);
                        objects.insert(dists_0[p].1);
                    }

                    if objects.len() > 11 {
                        let left = dists_0[&intersection[0]].0;
                        let right = dists[&intersection[0]].0;
                        let trans = left.sub(&right);

                        let c = Coord::new(trans.0, trans.1, trans.2);
                        if !scanner_dists.contains(&c) {
                            scanner_dists.push(c);
                            keep_going = true;
                        }

                        merge(&mut input[i], &all_objects, trans, left.system);
                        input[j].clear();
                        break;
                    }
                }
            }
        }
    }

    // Part 1 is total number of unique beacons, converted and added to the
    // first scanner list, MINUS the number of scanners, because we've added one beacon
    // per scanner at (0,0,0) for part 2.
    println!("Part 1: {}", input[0].len() - input.len());
    let scanners = input[0]
        .iter()
        .filter(|c| c.scanner)
        .copied()
        .collect::<Vec<Coord>>();

    // Part 2 is just the max Manhattan distance between those beacons that have the `scanner` tag.
    let mut part2 = 0;
    for a in 0..scanners.len() {
        for b in a + 1..scanners.len() {
            let first = scanners[a];
            let second = scanners[b];
            let dist = (first.a - second.a).abs()
                + (first.b - second.b).abs()
                + (first.c - second.c).abs();
            part2 = max(dist, part2);
        }
    }
    println!("Part 2: {part2}");
}
