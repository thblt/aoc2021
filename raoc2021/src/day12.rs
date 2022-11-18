use lib::*;
use std::collections::{HashMap,HashSet};

#[derive(PartialEq, Eq,Copy,Clone,Debug,Hash)]
struct Cave {
    idx: usize,
    large: bool
}

struct CaveSystem {
    next_idx: usize,
    names_for: HashMap<String,Cave>,
    names_rev: HashMap<Cave,String>,
    edges: Vec<(Cave,Cave)>
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        CaveSystem {
            next_idx: 0,
            names_for: HashMap::new(),
            names_rev: HashMap::new(),
            edges: vec!(),
        }
    }

    pub fn insert(&mut self, name: &str) -> Cave {
        let name = name.to_string();
        if let Some(idx) = self.names_for.get(&name) {
            *idx
        } else {
            let new = Cave {
                idx: self.next_idx,
                large: name.chars().nth(0).unwrap().is_uppercase(),
            };
            self.names_for.insert(name.clone(), new);
            self.names_rev.insert(new, name.clone());
            self.next_idx += 1;
            println!("New cave named {} (returns {}): {:?}",
                     &name,
                     self.cave_name(&new).unwrap(),
                     new);
            new
        }
    }

    pub fn connect(&mut self, a: &str, b: &str) {
        let a = self.insert(a);
        let b = self.insert(b);
        self.edges.push((a,b));
        self.edges.push((b,a));
    }

    pub fn next_caves(&self, from: &Cave) -> Vec<Cave> {
        self.edges.iter().filter(|p| &p.0 == from).map(|p| p.1).collect()
    }


    fn do_walk(&self, start: &Cave, goal: &Cave, revisit_small: bool, mut way: Vec<Cave>, ways: &mut Vec<Vec<Cave>> ) {
        // We're at start, want to go to `end`.  We enumerate possible
        // future moves, and explore them all.
        // println!("Walking from {}.", self.cave_name(start).unwrap());
        way.push(*start);

        if start==goal {
            ways.push(way);
            return
        }

        // println!("Keep walking from {}.", self.cave_name(start).unwrap());

        let no_go: Vec<&Cave> = way.iter().filter(|c| !c.large).collect();
        for next in self.next_caves(start) {
            if !no_go.contains(&&next) {
                self.do_walk(&next, goal, revisit_small, way.clone(), ways)

            } else if revisit_small && next.idx > 1 {
                // @FIXME This ^^^^^^^^^^^^^^^ is a trick: because we
                // insert start and end manually before we parse the
                // input, they get the IDs 0 and 1.
                self.do_walk(&next, goal, false, way.clone(), ways)
            }
        }
    }

    pub fn walk(&self, start: &Cave, goal: &Cave, revisit_small: bool) -> Vec<Vec<Cave>> {
        let mut ret: Vec<Vec<Cave>> = vec!();
        self.do_walk(start, goal, revisit_small, vec!(), &mut ret);
        ret
    }

    pub fn cave_name(&self, cave: &Cave) -> Option<&String> {
        self.names_rev.get(cave)
    }

    pub fn print_edges(&self) {
        for edge in &self.edges {
            println!("{} <-> {}",
                     self.cave_name(&edge.0).unwrap(),
                     self.cave_name(&edge.1).unwrap(),
            )
        }
    }
}

fn main() {
    let mut cs = CaveSystem::new();
    let start = cs.insert("start");
    let end = cs.insert("end");
    println!("reading…");

    for line in read_lines("../inputs/12.txt").unwrap() {
        let line = line.unwrap();
        let mut sides = line.split("-");
        let a = &sides.next().unwrap();
        let b = &sides.next().unwrap();
        cs.connect(a, b);
    }

    cs.print_edges();


    let paths = cs.walk(&start,&end, false);
    let mut counta = 0;
    for path in &paths {
        counta += 1;
        for cave in path.into_iter() {
            print!("{} ", cs.cave_name(&cave).unwrap())
        }
        println!("");
    }

    let paths = cs.walk(&start,&end, true);
    let mut countb = 0;
    for path in &paths {
        countb += 1;
        for cave in path.into_iter() {
            print!("{} ", cs.cave_name(&cave).unwrap())
        }
        println!("");
    }
    println!("{} paths found (part 1).", counta);
    println!("{} paths found (part 2).", countb);
}
