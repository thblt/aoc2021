use std::cmp::min;

const DIRAC_WINNING_SCORE: usize = 21;

/// A stupid tree
struct Tree<T> {
    node: T,
    children: Vec<Box<Tree<T>>>,
}

type GameTree = Tree<DiracGame>;

impl GameTree {
    fn new(p1_pos: usize, p2_pos: usize) -> Self {
        let root = DiracGame::init(p1_pos, p2_pos);
        Self::build(&root)
    }

    fn build(node: &DiracGame) -> Self {
        Self {
            node: node.clone(),
            children: Self::make_children(&node),
        }
    }

    /// Compute all children for this game state.  Each player rolls
    /// three dices at their turn, which means we end up in 27
    /// different universes.  But some roll outcomes (the *sum* of the
    /// three rolls) will be the same in different universes,
    /// follows:
    ///
    /// | Outcome | Universes |
    /// |---------+-----------|
    /// | 3       | 1         |
    /// | 4       | 3         |
    /// | 5       | 6         |
    /// | 6       | 7         |
    /// | 7       | 6         |
    /// | 8       | 3         |
    /// | 9       | 1         |

    fn make_children(of: &DiracGame) -> Vec<Box<Self>> {
        let mut ret: Vec<Box<GameTree>> = Vec::with_capacity(7);
        if of.p1_score < DIRAC_WINNING_SCORE && of.p2_score < DIRAC_WINNING_SCORE {
            for (roll, universes) in &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let mut next = of.clone();
                next.roll(*roll, *universes);
                ret.push(Box::new(Self::build(&next)));
            }

            // let mut next1 = of.clone();
            // next1.roll(1);
            // let r1 = Self::build(&next1);

            // let mut next2 = of.clone();
            // next2.roll(2);
            // let r2 = Self::build(&next2);

            // let mut next3 = of.clone();
            // next3.roll(3);
            // let r3 = Self::build(&next3);


        }
        ret
    }

    fn print(&self) {
        fn print(node: &GameTree, depth: usize) {
            println!("{}{}", String::from("· ").repeat(depth), node.node);
            for child in &node.children {
                print(&child, depth + 1);
            }
        }

        print(self, 0);
    }

    fn count_wins(&self, mut p1_wins: u128, mut p2_wins: u128) -> (u128, u128) {
        if self.children.len() > 0 {
            for child in &self.children {
                let (p1, p2) = child.count_wins(0, 0);
                p1_wins += p1;
                p2_wins += p2;
            }
        } else  if self.node.p1_score >= DIRAC_WINNING_SCORE {
            p1_wins += self.node.universes;
            } else {
                p2_wins += self.node.universes;
            }
        (p1_wins, p2_wins)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct DiracGame {
    /// The current halfmove.
    halfmove: u8,
    /// In how many universes are we?
    universes: u128,
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
}

impl std::fmt::Display for DiracGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move {}, score is {}-{}, positions {}-{}, player {} {} in {} universes.",
            self.halfmove / 2 + 1,
            self.p1_score,
            self.p2_score,
            self.p1_pos + 1,
            self.p2_pos + 1,
            if self.halfmove % 2 == 0 { 1 } else { 2 },
            if self.p1_score >= DIRAC_WINNING_SCORE || self.p2_score >= DIRAC_WINNING_SCORE {
                "LOST"
            } else {
                "to roll"
            },
            self.universes
        )
    }
}

impl DiracGame {
    fn init(p1_pos: usize, p2_pos: usize) -> Self {
        Self::new(0, 1, p1_pos, 0, p2_pos, 0)
    }

    fn new(
        halfmove: u8,
        universes: u128,
        p1_pos: usize,
        p1_score: usize,
        p2_pos: usize,
        p2_score: usize,
    ) -> Self {
        Self {
            halfmove,
            universes,
            p1_pos,
            p1_score,
            p2_pos,
            p2_score,
        }
    }

    /// Roll a dice *in place*
    fn roll(&mut self, roll: usize, universes: u128) {
        self.halfmove += 1;
        self.universes *= universes;
        if self.halfmove % 2 == 1 {
            // Player 1 plays at odd moves.
            self.p1_pos = (self.p1_pos + roll) % 10;
            self.p1_score += self.p1_pos + 1;
        } else {
            self.p2_pos = (self.p2_pos + roll) % 10;
            self.p2_score += self.p2_pos + 1;
        }
    }
}

struct Die {
    count: u64,
}

impl Die {
    fn new() -> Die {
        Die { count: 0 }
    }

    fn roll(&mut self) -> u64 {
        self.count += 1;
        (self.count - 1) % 100 + 1
    }

    fn rolls(&mut self, count: usize) -> u64 {
        let mut ret: u64 = 0;
        for _ in 0..count {
            ret += self.roll();
        }
        ret
    }
}

fn main_deterministic() {
    let mut d = Die::new();
    let mut p1 = 4;
    let mut p2 = 7;
    let mut p1_score = 0;
    let mut p2_score = 0;

    loop {
        p1 = (p1 + d.rolls(3)) % 10;
        p1_score += p1 + 1;
        if p1_score >= 1000 {
            break;
        }
        p2 = (p2 + d.rolls(3)) % 10;
        p2_score += p2 + 1;
        if p2_score >= 1000 {
            break;
        }
    }

    println!(
        "Challenge result (deterministic): {}",
        d.count * min(p1_score, p2_score)
    );
}

fn main_dirac() {
    // Computing the game tree.
    let game: GameTree = GameTree::new(4, 7);
    // game.print();
    let (p1_wins, p2_wins) = game.count_wins(0,0);
    println!("Challenge result (dirac): {}", std::cmp::max(p1_wins, p2_wins));
}

fn main() {
    main_deterministic();
    main_dirac();
}
