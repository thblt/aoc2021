#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn energy(&self) -> u64 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    name: String,
    occupants: Vec<Amphipod>,
    capacity: usize,
    exit_for: Option<Amphipod>,
    edges: Vec<Edge>,
    prev: Vec<Option<usize>>,
    dist: Vec<Option<u64>>,
    second_amphipod_counted: bool,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct Edge {
    direct: bool,
    cost: u64,
    blockers: Vec<usize>,
}

impl Edge {
    fn default() -> Self {
        Self {
            direct: false,
            cost: u64::MAX,
            blockers: vec![],
        }
    }

    fn default_vec(n: usize) -> Vec<Self> {
        std::iter::repeat(Edge::default()).take(n).collect()
    }
}

impl Node {
    fn new_sideroom(color: Amphipod, occupants: &[Amphipod]) -> Self {
        Self {
            name: format!("sideroom_{color:?}"),
            occupants: occupants.to_vec(),
            capacity: 1,
            exit_for: Some(color),
            edges: Edge::default_vec(9),
            prev: [None; 9].to_vec(),
            dist: [None; 9].to_vec(),
            second_amphipod_counted: false,
        }
    }

    fn new_reserve(capacity: usize, name: &str) -> Self {
        Self {
            name: name.to_string(),
            occupants: vec![],
            capacity,
            exit_for: None,
            edges: Edge::default_vec(9),
            prev: [None; 9].to_vec(),
            dist: [None; 9].to_vec(),
            second_amphipod_counted: false,
        }
    }

    fn edges(&self) -> Vec<(usize, Edge)> {
        self.edges
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, e)| e.direct)
            .collect()
    }

    fn occupant(&self) -> Option<Amphipod> {
        self.occupants.last().copied()
    }

    fn has_room(&self) -> bool {
        self.occupants.len() < self.capacity
    }

    fn is_empty(&self) -> bool {
        self.occupants.is_empty()
    }
}

fn dijkstra(graph: &mut [Node], start: usize) {
    fn next_node(graph: &[Node], queue: &mut Vec<usize>, start: usize) -> Option<usize> {
        queue.sort_by_key(|n| 0 - graph[*n].dist[start].unwrap() as isize);
        queue.pop()
    }
    let mut queue = vec![start];
    while let Some(node) = next_node(graph, &mut queue, start) {
        let dist = graph[node].dist[start].unwrap();
        let new_dist = dist + 2;

        for (other_id, _) in graph[node].edges() {
            let other_node = graph[other_id].clone();

            if other_node.dist[start].is_none() {
                queue.push(other_id);
            }
            if other_node.dist[start].is_none_or(|prev_dist| prev_dist > new_dist) {
                graph[other_id].dist[start] = Some(new_dist);
                graph[other_id].prev[start] = Some(node);
            }
        }
    }
}

fn make_graph() -> Vec<Node> {
    let mut graph: Vec<Node> = vec![];
    let mut create = |n: Node| {
        graph.push(n);
        graph.len() - 1
    };

    // Build the graph, first manually
    use Amphipod::*;
    // Exemple
    // let sr_a = create(Node::new_sideroom(A, &[A, B]));
    // let sr_b = create(Node::new_sideroom(B, &[D, C]));
    // let sr_c = create(Node::new_sideroom(C, &[C, B]));
    // let sr_d = create(Node::new_sideroom(D, &[A, D]));
    // My input
    let sr_a = create(Node::new_sideroom(A, &[C, D]));
    let sr_b = create(Node::new_sideroom(B, &[A, A]));
    let sr_c = create(Node::new_sideroom(C, &[B, C]));
    let sr_d = create(Node::new_sideroom(D, &[B, D]));
    let hw_left = create(Node::new_reserve(2, "hw_left"));
    let hw_right = create(Node::new_reserve(2, "hw_right"));
    let pause1 = create(Node::new_reserve(1, "pause_1"));
    let pause2 = create(Node::new_reserve(1, "pause_2"));
    let pause3 = create(Node::new_reserve(1, "pause_3"));

    for (a, b) in [
        (hw_left, pause1),
        (pause1, pause2),
        (pause2, pause3),
        (pause3, hw_right),
        (sr_a, hw_left),
        (sr_a, pause1),
        (sr_b, pause1),
        (sr_b, pause2),
        (sr_c, pause2),
        (sr_c, pause3),
        (sr_d, pause3),
        (sr_d, hw_right),
    ] {
        graph[a].edges[b].direct = true;
        graph[b].edges[a].direct = true;
    }

    // Run dijkstra and simplify as a list of edges with a list of "blockers"
    for end in 0..graph.len() {
        graph[end].dist[end] = Some(0);
        dijkstra(&mut graph, end);
        for start in 0..graph.len() {
            if end == start {
                continue;
            };
            let mut current = start;
            let cost = graph[start].dist[end].unwrap();
            // println!(
            //     "To go from {} to {} in {} steps",
            //     graph[start].name, graph[end].name, cost
            // );
            let mut blockers = vec![];
            loop {
                current = graph[current].prev[end].unwrap();
                if current == end {
                    break;
                }
                blockers.push(current);
                // println!("  Go through {}", graph[current].name);
            }
            graph[end].edges[start].blockers = blockers;
            graph[end].edges[start].cost = cost;
        }
    }

    graph
}

fn can_move(graph: &[Node], start: usize, dest: usize) -> bool {
    // We don't move in place
    start != dest
    // We don't leave a side room if it contains only amphipods of the right color.
    && graph[start].exit_for.is_none_or(|color| graph[start].occupants.iter().any(|c| *c != color))
    // We must move to or from a side room
    && (graph[start].exit_for.is_some() || graph[dest].exit_for.is_some())
    // The path must be free
    && graph[start].edges[dest]
        .blockers
        .iter()
        .all(|bl| graph[*bl].is_empty())
        // Our destination must have room
        && graph[dest].has_room()
}

fn legal_moves(graph: &[Node]) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    for start_node in (0..graph.len()).filter(|n| !graph[*n].is_empty()) {
        let occupant = graph[start_node].occupant().unwrap();
        // This is hardcoded, but the ID of the "siderooms" are the
        // usize values of the members of the Amphipod enum.  This is
        // why we can say "occupant as usize"
        if can_move(graph, start_node, occupant as usize) {
            // We MUST do final moves as soon as we can.
            return vec![(start_node, occupant as usize)];
        } else {
            for cand_dest in 4..graph.len() {
                if can_move(graph, start_node, cand_dest) {
                    ret.push((start_node, cand_dest));
                }
            }
        }
    }
    // ret.sort_by_key(|(s,d)| graph[*s].edges[*d].cost);
    ret
}

fn do_move(graph: &mut [Node], from: usize, dest: usize) -> u64 {
    let amphipod = graph[from].occupants.pop().unwrap();

    // Cost of that motion
    let mut cost = amphipod.energy() * graph[from].edges[dest].cost;

    // If we enter the end of an hallway, and an amphipod as already
    // present, we push it to the back of the hallway.  We thus add
    // two steps of *its* energy to the total cost, and mark that
    // we've handled it to avoid double counts (it may have already
    // been pushed by another amphipod that's since left)
    if graph[dest].exit_for.is_none()
        && !graph[dest].occupants.is_empty()
        && !graph[dest].second_amphipod_counted
    {
        cost += 2 * graph[dest]
            .occupants
            .iter()
            .map(Amphipod::energy)
            .sum::<u64>();
        // Mark
        graph[dest].second_amphipod_counted = true;
    }

    // Remove the mark
    if graph[from].occupants.is_empty() {
        graph[from].second_amphipod_counted = false;
    }

    // The cost of moving inside a sideroom (towards its end/bottom)
    // is already computed by prepare().  We don't even enter
    // siderooms: we just disappear.
    if graph[dest].exit_for.is_none() {
        graph[dest].occupants.push(amphipod);
    }
    cost
}

fn best_hope(graph: &[Node]) -> u64 {
    let mut ret = 0;
    for node in graph.iter().skip(4) {
        for occupant in &node.occupants {
            let dest = (*occupant) as usize;
            ret += node.edges[dest].cost * occupant.energy();
        }
    }
    ret
}

fn play(graph: Vec<Node>, base_cost: u64, best_cost: &mut u64) {
    if base_cost + best_hope(&graph) >= *best_cost {
        return;
    }

    if graph.iter().all(|n| n.is_empty()) {
        if base_cost < *best_cost {
            *best_cost = base_cost
        }
        return;
    }

    for (start, dest) in legal_moves(&graph) {
        let mut graph = graph.to_vec();
        let extra_cost = do_move(&mut graph, start, dest);
        play(graph, base_cost + extra_cost, best_cost);
    }
}

// Prepare a graph by computing its base cost (what it will cost in
// all cases for amphipods to reach the end of their exit side room)
// and pruning amphipods already in their final position.
fn prepare(graph: &mut [Node]) -> u64 {
    let mut ret = 0;
    let mut counts = [0; 4];
    for (i, node) in graph.iter_mut().enumerate().take(4) {
        // Prune amphipods already in their final position
        while node.occupants[0] as usize == i {
            node.occupants.remove(0);
        }
        //
        let occupation = node.occupants.len();
        for (nth, (amph, amph_num)) in node
            .occupants
            .iter()
            .map(|amph| (amph, (*amph) as usize))
            .enumerate()
        {
            // Energy this amphipod will spent going as deep
            // as possible in its EXIT room.
            ret += amph.energy() * counts[amph_num];
            counts[amph_num] += 1;
            // Extra energy this amphipod will spent by leaving
            // this room
            ret += (occupation - nth - 1) as u64 * amph.energy();
        }
    }
    ret
}

fn main() {
    let graph = make_graph();
    let mut energy = u64::MAX;
    let mut graph1 = graph.clone();
    let base_cost = prepare(&mut graph1);
    play(graph1, base_cost, &mut energy);
    println!("Part 1: {energy}");

    let mut graph = graph.clone();
    // Update graph for part 2
    use Amphipod::*;
    let part2 = [[D, D], [C, B], [B, A], [A, C]];
    for i in 0..4 {
        graph[i].occupants.extend(part2[i].iter());
        graph[i].occupants.swap(1, 3);
    }

    energy = u64::MAX;
    let base_cost = prepare(&mut graph);
    play(graph, base_cost, &mut energy);
    println!("Part 2: {energy}");
}
