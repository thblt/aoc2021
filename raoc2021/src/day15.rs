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
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

fn dijkstra(graph: &Graph<usize>, start: usize, to: usize) -> HashMap<usize, usize> {
    let mut dist: HashMap<usize, u32> = HashMap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();
    let mut nodes = graph.nodes.clone();

    dist.insert(start, 0);

    println!("Now to the main loop;");
    loop {
        // println!("Dists are {:?}", dist);
        let u = nodes
            .iter()
            .filter(|node| dist.get(node).is_some())
            .min_by_key(|node| dist.get(node).unwrap());

        if u.is_none() {
            break;
        }
        let u = *u.unwrap();

        nodes.remove(&u);

        for edge in graph.edges.iter().filter(|e| e.is_from(&u)) {
            let v = edge.b;
            if nodes.contains(&v) {
                let alt = dist.get(&u).unwrap() + edge.cost;
                if let Some(v_dist) = dist.get(&v) {
                    if alt < *v_dist {
                        dist.insert(v, alt);
                        prev.insert(v, u);
                    }
                } else {
                    dist.insert(v, alt);
                    prev.insert(v, u);
                    if v % 50 == 0 {
                        println!("Explored {}: {}", v, alt);
                    }
                }
            }
        }
    }
    println!("{:?}", dist.get(&to));
    prev
}

fn main() {
    let mut maze = multiply_input(&read_input());
    let mut graph = Graph::<usize>::from_vec2d(&maze);

    let from = maze.to_index((0, 0));
    let to = maze.to_index(((maze.height() - 1) as isize, (maze.width() - 1) as isize));
    let prevs = dijkstra(&mut graph, from, to);

    let mut current = to;
    loop {
        maze.vec[current] = 10;
        if !prevs.contains_key(&current) {
            break;
        }
        current = *prevs.get(&current).unwrap();
    }
    maze.draw();
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd, Ord)]
struct Edge<T: Debug + PartialEq + Eq + Copy + Hash + PartialOrd> {
    a: T,
    b: T,
    cost: u32,
}

impl<T: Debug + PartialEq + Eq + Copy + Hash + PartialOrd + Ord> Edge<T> {
    pub fn new(a: T, b: T, cost: u32) -> Edge<T> {
        Edge { a, b, cost }
    }

    pub fn is_from(&self, t: &T) -> bool {
        self.a == *t
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Graph<E: Debug + Copy + Eq + PartialEq + PartialOrd + Hash> {
    nodes: HashSet<E>,
    edges: HashSet<Edge<E>>,
}

impl<E: Copy + Eq + PartialEq + Hash + PartialOrd + Debug> Graph<E> {
    fn new() -> Graph<E> {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn from_vec2d(vec: &Vec2D<u8>) -> Graph<usize> {
        if vec.height() != vec.width() {
            panic!("I wanted a square graph!");
        }

        let mut ret = Graph::<usize>::new();

        for i in 0..vec.width() as isize {
            for j in 0..=i {
                for (x, y) in [(i, j), (j, i)] {
                    let this_cost = vec[(x, y)] as u32;
                    let id = vec.to_index((x, y));
                    ret.nodes.insert(id);
                    if x > 0 {
                        ret.edges.insert(Edge::new(
                            id,
                            vec.to_index((x - 1, y)),
                            vec[(x - 1, y)] as u32,
                        ));
                        ret.edges
                            .insert(Edge::new(vec.to_index((x - 1, y)), id, this_cost));
                    }
                    if y > 0 {
                        ret.edges.insert(Edge::new(
                            id,
                            vec.to_index((x, y - 1)),
                            vec[(x, y - 1)] as u32,
                        ));
                        ret.edges
                            .insert(Edge::new(vec.to_index((x, y - 1)), id, this_cost));
                    }
                }
            }
        }
        ret
    }
}

fn multiply_input(vec: &Vec2D<u8>) -> Vec2D<u8> {
    let w1 = vec.width() as isize;
    let h1 = vec.height() as isize;
    let w2 = w1 * 5_isize;
    let h2 = h1 * 5_isize;
    let mut ret = Vec2D::<u8>::new(w2 as usize, h2 as usize, 0);

    for x in 0..w2 {
        for y in 0..h2 {
            let nx = x / w1;
            let ny = y / h1;
            let mut val = vec[(x % w1, y % h1)] + (nx as u8) + (ny as u8);
            if val > 9 {
                val %= 9
            }
            ret[(x as isize, y as isize)] = val
        }
    }
    ret
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
