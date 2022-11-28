// Bon, ce truc de modification de position signifie simplement que
// pour chaque scanner, l'ordre des coordonnées x,y,z est variable (il
// y a six ordre possible: xyz xzy yxz yzx zxy zyx) et chaque axe peut
// être inversé, ie +4 peut être -4.
//
// Si un scanner est un cube, il peut avoir n'importe laquelle de ses
// six faces en haut, et à partir de ces six positions de départ,
// subir quatre rotations selon l'axe vertical.

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use lib::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

    /// Like rotate(), but returns a new CoordSystem.
    fn rotated(&self, along: Axis, forward: bool) -> CoordSystem {
        let mut ret = self.clone();
        ret.rotate(along, forward);
        ret
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    a: i32,
    b: i32,
    c: i32,
    system: CoordSystem,
}

impl Coord {
    fn new(a: i32, b: i32, c: i32) -> Coord {
        Coord {
            a,
            b,
            c,
            system: CoordSystem::default(),
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
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{}@{})", self.a, self.b, self.c, self.system)
    }
}

fn read_input(file: &str) -> Vec<Vec<Coord>> {
    let mut ret: Vec<Vec<Coord>> = vec![];
    let mut current: Vec<Coord> = vec![];
    let mut start = true;

    for line in read_lines(file).unwrap() {
            let line = line.unwrap();
            if start {
                // Skip first line
start = false
        } else if line.is_empty() {
            start = true;
                ret.push(current);
                current = vec![];
        } else {
            let parts: Vec<i32> = line.split(",").map(|l| l.parse::<i32>().unwrap()).collect();
            current.push(Coord {
                a: parts[0],
                b: parts[1],
                c: parts[2],
                system: CoordSystem::default(),
            });
        }
    }
    ret.push(current);
    ret
}

fn pair_distances(cs: &Vec<Coord>) -> HashMap<(i32, i32, i32), (usize, usize)> {
    let mut ret: HashMap<(i32, i32, i32), (usize, usize)> = HashMap::new();
    for a in 0..cs.len() {
        for b in a+1..cs.len() {
            ret.insert(cs[a].sub(&cs[b]), (a, b));
        }
    }
    ret
}

fn dump_input(input: &Vec<Vec<Coord>>) {
    for s in 0..input.len() {
        if s > 0 {
            println!();
        }
        println!("--- scanner {} ---", s);
        for b in &input[s] {
            println!("{},{},{}", b.a, b.b, b.c);
        }
    }
}

fn store_equiv(db: &mut Vec<HashSet<(usize, usize)>>, a: (usize, usize), b: (usize, usize)) {
    for mut set in &mut db.into_iter() {
        if set.contains(&a) {
            set.insert(b);
            return;
        } else if set.contains(&b) {
            set.insert(a);
            return;
        }
    }
    let mut new = HashSet::new();
    new.insert(a);
    new.insert(b);
    db.push(new);
}

fn main() {
    // Method.  The goal is to merge objects from all scanners into scanner 0.
    //
    //  - Traverse all possible scanner pairs until we find a pair
    //  (A,B) with at least twelve overlapping objects.
    //
    //  - Translate coordinates in B to A (that is, take any
    //  overlapping object, substract its coordinates in A to those in
    //  B, add the result to all coordinates of objects in B) and
    //  merge the two lists in one, A, removing duplicates.  Clear
    //  list B and remove it.
    //
    // - Restart at beginning until there's only one list remaining.
    //
    // - Count objects in list for part 1.
    //
    let input = read_input("../inputs/19.txt");
    let mut best;
    let mut best_cs = CoordSystem::default();
    let mut best_dists: HashMap<(i32, i32, i32), (usize, usize)> = HashMap::new();
    let mut best_keys: Vec<(i32, i32, i32)> = vec![];
    let mut equivs: Vec<HashSet<(usize, usize)>> = vec![];

    for i in 0..input.len() {
        println!("Scanner {} has seen {} objects", i, input[i].len());
        let dists_0 = pair_distances(&input[i]);
        let dists_0_only: Vec<&(i32, i32, i32)> = dists_0.keys().collect();
        for j in i+1..input.len() {
            best = 11;
            for cs in CoordSystem::all() {
                let dists = pair_distances(&input[j].iter().map(|c| c.translate(cs)).collect());
                let intersection: Vec<(i32, i32, i32)> = dists
                    .keys()
                    .filter(|v| dists_0_only.contains(v))
                    .map(|v| *v)
                    .collect();

                let mut set: HashSet<usize> = HashSet::new();
                for p in &intersection {
                    set.insert(dists_0[&p].0);
                    set.insert(dists_0[&p].1);
                }

                if set.len() > best {
                    best = set.len();
                    best_keys = intersection.clone();
                        best_dists = dists.clone();
                        best_cs = cs;
                    }
            }
    if best > 11 {
        println!(
            "Scanners {} and {}, the latter interpreted in {}, share {} common objects.",
            i, j, best_cs, best
        );
        for k in &best_keys {
            let left = dists_0[k];
            let right = best_dists[k];
            store_equiv(&mut equivs, (i, left.0), (j, right.0));
            store_equiv(&mut equivs, (i, left.1), (j, right.1));
        }
    }
       }
    }
    // for i in 0..input.len() {
    //     let dists_0 = pair_distances(&input[i]);
    //     let dists_0_only: Vec<&(i32, i32, i32)> = dists_0.keys().collect();
    //     for j in i + 1..input.len() {
    //         best = 11;
    //         for cs in CoordSystem::all() {
    //             let dists = pair_distances(&input[j].iter().map(|c| c.translate(cs)).collect());
    //             let intersection: Vec<(i32, i32, i32)> = dists
    //                 .keys()
    //                 .filter(|v| dists_0_only.contains(v))
    //                 .map(|v| *v)
    //                 .collect();

    //             if intersection.len() > best {
    //                 best = intersection.len();
    //                 best_keys = intersection.clone();
    //                 best_dists = dists.clone();
    //                 best_cs = cs;
    //             }
    //         }
    //         if best > 11 {
    //             println!(
    //                 "Scanner {} against {} translated to {}, {} potential common objects:",
    //                 i, j, best_cs, best
    //             );
    //             for k in &best_keys {
    //                 // println!("SToring {:?}", k);
    //                 let left = dists_0[k];
    //                 let right = best_dists[k];
    //                 store_equiv(&mut equivs, (i,left.0), (j,right.0));
    //                 store_equiv(&mut equivs, (i,left.1), (j,right.1));
    //             }
    //         }
    //     }
    // }
    println!("Total: {:?}", equivs.len());
}
