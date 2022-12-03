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

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug, Hash)]
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
    let mut current: HashSet<Coord> = HashSet::new();
    let mut start = true;

    for line in read_lines(file).unwrap() {
        let line = line.unwrap();
        if start {
            // Skip first line
            start = false
        } else if line.is_empty() {
            start = true;
            ret.push(current);
            current = HashSet::new();
        } else {
            let parts: Vec<i32> = line.split(",").map(|l| l.parse::<i32>().unwrap()).collect();
            current.insert(Coord {
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

fn store_equiv(db: &mut Vec<HashSet<(Coord, Coord)>>, a: (Coord, Coord), b: (Coord, Coord)) {
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

/// Merge FROM into TO, draining FROM, adding TRANS first.
fn merge(to: &mut HashSet<Coord>, from: &HashSet<Coord>, trans: (i32, i32, i32), cs: CoordSystem) {
    // println!("MERGE init sum {}+{}={}", from.len(), to.len(), from.len() + to.len());
    for c in from {
        let mut c = c.clone();
        c.add(trans);
        c.system = cs;
        to.insert(c);
    }
    // println!("DONE final sum {}", to.len());
}

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
    let mut input = read_input("../inputs/19.txt");
    let mut best_cs = CoordSystem::default();
    let mut best_dists: HashMap<(i32, i32, i32), (Coord, Coord)> = HashMap::new();
    let mut best_keys: Vec<(i32, i32, i32)> = vec![];
    let mut objects: HashSet<Coord> = HashSet::new();
    let mut all_objects: HashSet<Coord> = HashSet::new();
    let mut scanner_dists: Vec<Coord> = vec![];

    let mut keep_going = true;
    while keep_going {
        println!("STEP");
        keep_going = false;
        for i in 0..input.len() {
            // println!("Scanner {} has seen {} objects", i, input[i].len());
            let dists_0 = pair_distances(&input[i]);
            let dists_0_only: Vec<(i32, i32, i32)> = dists_0.keys().map(|c| *c).collect();
            for j in i + 1..input.len() {
                for cs in CoordSystem::all() {
                    let all_objects = input[j]
                        .iter()
                        .map(|c| c.translate(cs))
                        .collect::<HashSet<Coord>>();
                    let dists = pair_distances(&all_objects);
                    let intersection: Vec<(i32, i32, i32)> = vec_inter(
                        dists.keys().map(|c| *c).collect::<Vec<(i32, i32, i32)>>(),
                        dists_0_only.clone(),
                    );

                    objects.clear();
                    for p in &intersection {
                        objects.insert(dists_0[&p].0);
                        objects.insert(dists_0[&p].1);
                    }

                    // println!("{}", objects.len());
                    if objects.len() > 11 {
                        let left = dists_0[&intersection[0]].0;
                        let mut right = dists[&intersection[0]].0;
                        let trans = left.sub(&right);

                        let c = Coord::new(trans.0, trans.1, trans.2);
                        if !scanner_dists.contains(&c) {
                            scanner_dists.push(c);
                            keep_going = true;
                        }
                        println!("{} -> {} = {:?}", i, j, trans);

                        // merge(&mut input[i], &best_all_objects, trans, left.system);
                        break;
                    }
                }
            }
        }
    }

    let mut best = 0;
    for left in &scanner_dists {
        for right in &scanner_dists {
            let dist =
            // left.a.abs() + left.b.abs() + left.c.abs();
            abs_diff(left.a, right.a) + abs_diff(left.b, right.b) + abs_diff(left.c, right.c);
        if dist > best {
            best = dist;
        }
    }
}

    println!("Manhattan max: {:?}", best);
}
