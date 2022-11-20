// This is an optimization problem: we don't actually need to compute
// the full polymer, but just the final property, ie the difference
// between the most and least common elements.  To bring complexity
// down to something approaching θ(n), we only process pairs of
// elements, and ignore the overall structure of the polymer.  Thus,
// we parse the example input NNCB as:

// NN=1; NC=1; CB=1.
//
// Applying the rule NN -> C means that for each NN, we produce a NC
// and a CN.  Therefore, after applying this rule, we now have:
//
// NN=0; NC=2; CB=1, CN=1.
//
// and so on.
//
// To get the final count, we simply break those pairs and add their
// value to the total sum of each element.  Eg, CB=68 means we add 68
// to the total C count and to the total B count.
//
// Because each element *but the first and the last of the original
// polymer* will appear twice, we add one of those to the total count,
// then divide each value by 2 to get the actual count.  The rest is
// trivial.

use lib::*;
use std::collections::HashMap;

type Pair = (u8,u8);
/// A state is simply a count of pairs (we don't preserve structure)
type State = HashMap<Pair, u64>;
type Rules = HashMap<Pair, u8>;
type Count = HashMap<u8, u64>;

fn compute() -> u64 {
    let (raw_state, rules) = read_input();
    // We need to preserve the first and the last component of the
    // polymer.  Since we're later going to break the polymer into a
    // simple list of pairs, losing structure, each element *but the
    // first and last* will be counted twice.  We need to restore
    // these two befor the final count.
    let first = raw_state.first().unwrap();
    let last = raw_state.last().unwrap();

    // We break the initial state into pairs.
    let mut state: State = State::new();
    for i in 0..raw_state.len()-1 {
        let key = (raw_state[i],raw_state[i+1]);
        maybe_init(&mut state, key);
        *state.get_mut(&key).unwrap() += 1;
    }
    // We alternate between two state maps to avoid allocating memory
    // at each iteration.
    let mut state_odd = state.clone();
    let mut state_last = &state;
    for i in 1..=40 {
        if i % 2 == 0 {
            evolve(&state, &mut state_odd, &rules);
            state_last = &state_odd;
        } else {
            evolve(&state_odd, &mut state, &rules);
            state_last = &state;
        }
    }
    let count = count_elems(&state_last, *first, *last);
    print_count(&count);
    count.values().max().unwrap()-count.values().min().unwrap()
}

fn evolve(prev: &State, next: &mut State, rules: &Rules) {
    // We reset the next state.
    for v in next.values_mut() {
        *v = 0;
    }

    // Then, we iterate over the previous map.
    for (pair@(l, r), count) in prev {
        // @FIXME This is nitpicking, but that rules.get() in a loop
        // could probably be optimized away, eg by having the rule map
        // *and* the pairs map be defined for all possible pairs ---
        // there's just 100 of them.
        if let Some(new) =  rules.get(&pair) {
            maybe_init(next, (*l,*new));
            *next.get_mut(&(*l,*new)).unwrap() += count;
            maybe_init(next, (*new,*r));
            *next.get_mut(&(*new,*r)).unwrap() += count;
        } else {
            *next.get_mut(&pair).unwrap() += count;
        }
    }
}

fn count_elems(state: &State, first: u8, last: u8) -> Count{
    let mut ret: Count = Count::new();
    ret.insert(first, 1);
    ret.insert(last, 1);
    for ((l,r), count) in state {
        // Fully process left, then right, in case left==right.
        let l_count: u64 = *ret.get(l).unwrap_or(&0);
        ret.insert(*l, count+l_count);
        // Right
        let r_count: u64 = *ret.get(r).unwrap_or(&0);
        ret.insert(*r, count+r_count);
    }
    for v in ret.values_mut() {
        *v /= 2
    }
    ret
}

fn print_count(count: &Count) {
    for (k,v) in count.iter() {
        println!("{} = {}", show1(k), v);
    }
}

fn maybe_init(state: &mut State, key: Pair) {
    if !state.contains_key(&key) {
        state.insert(key, 0);
    }
}

/// The input "parser".
fn read_input() -> (Vec<u8>,Rules) {
    let mut input = read_lines("../inputs/14.txt").unwrap();
    let mut rules: HashMap<(u8,u8),u8> = HashMap::new();

    // Read initial state
    let state = input.next().unwrap().unwrap().into_bytes();
    println!("Starting from {:?}", show(&state));

    // Read rules
    for raw in input.skip(1) {
        let line = raw.unwrap().into_bytes();
        let pair = (line[0], line[1]);
        let new = line[6];
        rules.insert(pair, new);
    }

    (state,rules)
}

///////////////////////////////////////////////////////////////////////
// STUPID FIRST VERSION (PART 1) //////////////////////////////////////
///////////////////////////////////////////////////////////////////////

fn stupid_evolve(state: &[u8], rules: &HashMap<(u8,u8),u8>) -> Vec<u8> {
    let mut ret: Vec<u8> = vec![state[0]];

    for i in 0..state.len()-1 {
        // ret.push(state[i]);
        if let Some(x) = rules.get(&(state[i], state[i+1])) {
            ret.push(*x);
        }
        ret.push(state[i+1]);
    }
    ret
}

fn stupid_compute() {
    let mut input = read_lines("../inputs/14.txt").unwrap();
    let mut rules: HashMap<(u8,u8),u8> = HashMap::new();

    // Read initial state
    let mut state = input.next().unwrap().unwrap().into_bytes();
    println!("Starting from {:?}", show(&state));

    // Read rules
    for raw in input.skip(1) {
        let line = raw.unwrap().into_bytes();
        let pair = (line[0], line[1]);
        let new = line[6];
        rules.insert(pair, new);
    }

    for i in 1..=10 {
        state = stupid_evolve(&state.as_slice(), &rules);
        println!("After {} evolutions, it measures {}", i, state.len());
    }
    state.sort();
    let mut current = state[0];
    let mut count = 0;
    let mut counts = vec!();
    for b in state {
        if b == current {
            count += 1
        } else {
            counts.push(count);
            current = b;
            count = 1;
        }
    }
    counts.push(count);
    println!("{}", counts.iter().max().unwrap()-counts.iter().min().unwrap());
}

///////////////////////////////////////////////////////////////////////
// UTILITIES                                                         //
///////////////////////////////////////////////////////////////////////

fn show(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).unwrap().to_string()
}

fn show1(bytes: &u8) -> String {
    std::str::from_utf8(&[*bytes]).unwrap().to_string()
}

fn main() {
    stupid_compute();
    println!("Part 2: {}", compute());
}

//
