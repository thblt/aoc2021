use lib::*;
use std::fmt::Display;

#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Ord)]
enum Atom {
    Literal(u32),
    Pair(Box<(Atom, Atom)>),
    Void,
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Literal(v) => v.fmt(f),
            Atom::Pair(b) => {
                write!(f, "[")?;
                write!(f, "{}", b.0)?;
                write!(f, ",")?;
                write!(f, "{}", b.1)?;
                write!(f, "]")
            }
            Atom::Void => todo!(),
        }
    }
}

impl Atom {
    fn as_pair(&self) -> Option<&Box<(Atom, Atom)>> {
        if let Self::Pair(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the atom is [`Literal`].
    ///
    /// [`Literal`]: Atom::Literal
    #[must_use]
    fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    /// Returns `true` if the atom is [`Pair`].
    ///
    /// [`Pair`]: Atom::Pair
    #[must_use]
    fn is_pair(&self) -> bool {
        matches!(self, Self::Pair(..))
    }

    fn reduce<'a>(&'a mut self, mut previous: &'a mut Atom, depth: usize) {
        // let mut previous;
        if depth < 3 {
            match self {
                Atom::Literal(v) if *v > 10 => *previous = Atom::new(23),
                Atom::Pair(pair) => {
                    if pair.1.is_literal() {
                        previous = &mut pair.1
                    } else if pair.0.is_literal() {
                        previous = &mut pair.0
                    }
                    pair.0.reduce(previous, depth + 1);
                    pair.1.reduce(previous, depth + 1);
                }
                _ => {} //     Atom::Void => return,
            }
        } else {

        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
struct AtomParserState {
    left: bool,
    lhs: Atom,
    rhs: Atom,
}

impl AtomParserState {
    fn new() -> AtomParserState {
        AtomParserState {
            left: true,
            lhs: Atom::Void,
            rhs: Atom::Void,
        }
    }

    fn atom(&mut self) -> &Atom {
        if self.left {
            &mut self.lhs
        } else {
            &mut self.rhs
        }
    }

    fn set_atom(&mut self, atom: Atom) {
        if self.left {
            self.lhs = atom
        } else {
            self.rhs = atom
        }
    }

    fn finalize(&self) -> Atom {
        Atom::Pair(Box::new((self.lhs.clone(), self.rhs.clone())))
    }

    fn comma(&mut self) {
        self.left = false;
    }

    fn update_literal(&mut self, digit: char) {
        let acc = match &mut self.atom() {
            Atom::Literal(acc) => *acc,
            _ => 0,
        };

        // let mut atom = self.atom();
        self.set_atom(Atom::Literal(acc * 10 + (read_digit(digit) as u32)));
    }
}

impl Atom {
    fn new(v: u32) -> Atom {
        Atom::Literal(v)
    }

    fn add(&self, other: &Atom) -> Atom {
        Atom::Pair(Box::new((self.clone(), other.clone())))
    }

    fn parse(s: &str) -> Atom {
        let mut stack: Vec<AtomParserState> = vec![];
        for c in String::from(s).chars() {
            match c {
                '[' => {
                    stack.push(AtomParserState::new());
                }
                ']' => {
                    let atom = stack.pop().unwrap().finalize();
                    if stack.is_empty() {
                        return atom;
                    } else {
                        stack.last_mut().unwrap().set_atom(atom);
                    }
                }
                ',' => stack.last_mut().unwrap().comma(),
                _ => stack.last_mut().unwrap().update_literal(c),
            }
        }

        println!("{:?}", stack);
        todo!()
    }
}

fn main() {
    let mut atom = Atom::parse("[1,[[12,0],18]]");
    println!("{}", atom);
    atom.reduce(&mut atom.clone(),  0);
    println!("{}", atom);
}
