// This was a space complexity problem, the solution is to represent the reactor
// as a list of cuboids, representing a range of cubes in the on state (that
// list is initially empty).  The only catch is that those cuboids are on Z³,
// not R³, so handling intersection /partition of cubes require special care:
// cubes that share even a vertice have a non empty intersection, a cuboid
// defined by two identical points eg (0,0,0)-(0,0,0) has a volume of 1 (the
// "cube" at 0,0,0)
use sscanf::sscanf;
use std::cmp::{max, min};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Cuboid {
    a: Point3,
    b: Point3,
}

/// A cuboid in ℤ³.  
impl Cuboid {
    pub fn intersects(&self, other: &Cuboid) -> bool {
        fn intersects1(a: (i64, i64), b: (i64, i64)) -> bool {
            if a.0 > b.0 {
                intersects1(b, a)
            } else {
                b.0 <= (a.1)
            }
        }
        intersects1((self.a.x, self.b.x), (other.a.x, other.b.x))
            && intersects1((self.a.y, self.b.y), (other.a.y, other.b.y))
            && intersects1((self.a.z, self.b.z), (other.a.z, other.b.z))
    }

    // Split two intersecting cuboids into a vector of non-intersecting Cuboids.
    fn partition(&self, other: &Cuboid) -> Vec<Cuboid> {
        assert!(self.intersects(other));
        let mut ret = vec![];

        let mut xs = [self.a.x, self.b.x, other.a.x, other.b.x];
        let mut ys = [self.a.y, self.b.y, other.a.y, other.b.y];
        let mut zs = [self.a.z, self.b.z, other.a.z, other.b.z];

        xs.sort();
        ys.sort();
        zs.sort();

        fn ranges(values: &[i64; 4]) -> Vec<(i64, i64)> {
            let a = values[0];
            let b = values[1];
            let c = values[2];
            let d = values[3];

            if a == b && c == d {
                vec![(a, d)]
            } else if a == b {
                vec![(b, c), (c + 1, d)]
            } else if c == d {
                vec![(a, b - 1), (b, d)]
            } else {
                vec![(a, b - 1), (b, c), (c + 1, d)]
            }
        }
        for (x1, x2) in ranges(&xs) {
            for (y1, y2) in ranges(&ys) {
                for (z1, z2) in ranges(&zs) {
                    let cuboid = Cuboid {
                        a: Point3::new(x1, y1, z1),
                        b: Point3::new(x2, y2, z2),
                    };
                    if self.intersects(&cuboid) || other.intersects(&cuboid) {
                        ret.push(cuboid);
                    }
                    // }
                }
            }
        }
        ret
    }

    /// Like partition, but only returns cuboids that are part of self and only self.
    pub fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        self.partition(other)
            .drain(..)
            .filter(|c| !c.intersects(other))
            .collect()
    }

    pub fn intersection(&self, other: &Cuboid) -> Cuboid {
        let mut xs = [self.a.x, self.b.x, other.a.x, other.b.x];
        let mut ys = [self.a.y, self.b.y, other.a.y, other.b.y];
        let mut zs = [self.a.z, self.b.z, other.a.z, other.b.z];
        xs.sort();
        ys.sort();
        zs.sort();
        Cuboid {
            a: Point3::new(xs[1], ys[1], zs[1]),
            b: Point3::new(xs[2], ys[2], zs[2]),
        }
    }

    /// Return the points that make this cuboid
    pub fn explode(&self) -> Vec<Point3> {
        let mut ret = vec![];

        for x in self.a.x..=self.b.x {
            for y in self.a.y..=self.b.y {
                for z in self.a.z..=self.b.z {
                    ret.push(Point3::new(x, y, z));
                }
            }
        }
        ret
    }

    /// Determines whether point is part of this cuboid.
    pub fn contains(&self, point: &Point3) -> bool {
        fn in_bounds(a: i64, b: i64, cand: i64) -> bool {
            cand >= min(a, b) && cand <= max(a, b)
        }
        in_bounds(self.a.x, self.b.x, point.x)
            && in_bounds(self.a.y, self.b.y, point.y)
            && in_bounds(self.a.z, self.b.z, point.z)
    }

    pub fn width(&self) -> i64 {
        1 + (self.a.x - self.b.x).abs()
    }

    pub fn height(&self) -> i64 {
        1 + (self.a.y - self.b.y).abs()
    }

    pub fn depth(&self) -> i64 {
        1 + (self.a.z - self.b.z).abs()
    }

    pub fn volume(&self) -> i64 {
        self.width() * self.height() * self.depth()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Command {
    toggle: bool,
    cuboid: Cuboid,
}

fn turn_on(reactor: &[Cuboid], target: Cuboid) -> Vec<Cuboid> {
    // To turn a target on, we substract from it every cuboid in the reactor, and add what remains to the reactor.
    let mut ret = reactor.to_vec();
    let mut targets = vec![target];

    'targets: while let Some(target) = targets.pop() {
        for cuboid in reactor {
            if cuboid.intersects(&target) {
                targets.extend(target.subtract(cuboid));
                continue 'targets;
            }
        }
        // If we're here, there was no intersection, we just push the target to the output.
        ret.push(target);
    }
    ret
}

fn turn_off(reactor: &[Cuboid], target: Cuboid) -> Vec<Cuboid> {
    // To turn a target off, we simply substract it from all the cuboids in the reactor.
    let mut ret = vec![];
    for group in reactor {
        if group.intersects(&target) {
            ret.extend(group.subtract(&target));
        } else {
            ret.push(*group);
        }
    }
    ret
}

fn main() {
    let commands = read_input();
    let mut reactor: Vec<Cuboid> = vec![];
    for command in commands {
        reactor = if command.toggle {
            turn_on(&reactor, command.cuboid)
        } else {
            turn_off(&reactor, command.cuboid)
        }
    }
    let part1 = Cuboid {
        a: Point3::new(-50, -50, -50),
        b: Point3::new(50, 50, 50),
    };
    println!(
        "Part 1: {}",
        reactor
            .iter()
            .map(|c| if c.intersects(&part1) {
                c.intersection(&part1).volume()
            } else {
                0
            })
            .sum::<i64>()
    );
    println!(
        "Part 2: {}",
        reactor.iter().map(|c| c.volume()).sum::<i64>()
    );
}

/// Visually test the partition and intersection functions.
pub fn test_2d((x1, y1, x2, y2): (i64, i64, i64, i64), (x3, y3, x4, y4): (i64, i64, i64, i64)) {
    use ansi_term::Colour::{Black as CB, Blue as C1, Green as CI, Yellow as C2};
    let c1 = Cuboid {
        a: Point3::new(x1, y1, 0),
        b: Point3::new(x2, y2, 0),
    };
    let c2 = Cuboid {
        a: Point3::new(x3, y3, 0),
        b: Point3::new(x4, y4, 0),
    };
    let xmax = *[c1.a.x, c1.b.x, c2.a.x, c2.b.x].iter().max().unwrap();
    let ymax = *[c1.a.y, c1.b.y, c2.a.y, c2.b.y].iter().max().unwrap();
    let xmin = *[c1.a.x, c1.b.x, c2.a.x, c2.b.x].iter().min().unwrap();
    let ymin = *[c1.a.y, c1.b.y, c2.a.y, c2.b.y].iter().min().unwrap();
    let part = c1.partition(&c2);
    for y in ymin..=ymax + ymin.abs() {
        for x in xmin..=xmax + xmin.abs() {
            let here = Point3::new(x, y, 0);
            let in1 = c1.contains(&here);
            let in2 = c2.contains(&here);
            let color = ansi_term::Style::new().on(match (in1, in2) {
                (true, true) => CI,
                (true, false) => C1,
                (false, true) => C2,
                (false, false) => CB,
            });
            let part_id = part
                .iter()
                .enumerate()
                .filter_map(|(n, c)| if c.contains(&here) { Some(n) } else { None })
                .collect::<Vec<usize>>();
            assert!(part_id.len() < 2);

            let part_symbol = if let Some(n) = part_id.first() {
                color.paint(format!("{n}"))
            } else {
                color.paint("·".to_string())
            };
            print!("{part_symbol}");
        }
        if y == 0 {
            print!(
                " Partitions: {}, Surfaces: blue: {}, yellow: {}",
                part.len(),
                c1.volume(),
                c2.volume()
            );
        }
        println!();
    }
    let subst = c1.subtract(&c2);
    for y in 0..=ymax + ymin {
        for x in 0..=xmax + xmin {
            let here = Point3::new(x, y, 0);
            if subst.iter().any(|c| c.contains(&here)) {
                print!("█");
            } else if c2.contains(&here) {
                print!("░");
            } else {
                print!("·");
            }
        }
        if y == 0 {
            print!("Substraction test",);
        }
        println!();
    }
}

fn read_input() -> Vec<Command> {
    let mut ret = vec![];
    for line in lib::read_lines("../inputs/22.txt").unwrap() {
        let line = line.unwrap();
        let (toggle, x1, x2, y1, y2, z1, z2) = sscanf!(
            line,
            "{String} x={i64}..{i64},y={i64}..{i64},z={i64}..{i64}"
        )
        .unwrap();
        let toggle = toggle == "on";
        ret.push(Command {
            toggle,
            cuboid: Cuboid {
                a: Point3::new(x1, y1, z1),
                b: Point3::new(x2, y2, z2),
            },
        });
    }
    ret
}
