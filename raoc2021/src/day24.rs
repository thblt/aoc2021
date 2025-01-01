// This is unnecessarily brute force.  We could:
//
//  - At least not traverse the full list, but reverse for part 1, and
// in both cases halt at the first positive result.
//
// - Better: express the mathematical constraint, it should be easy.

use std::fmt::Display;

type Word = i64;

struct ALU {
    w: Word,
    x: Word,
    y: Word,
    z: Word,

    input: Vec<Word>,
}

impl ALU {
    fn new(input: &[Word]) -> Self {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input: input.to_vec(),
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        for instr in program {
            instr.run(self);
        }
    }

    fn optimize1(program: &[Instruction]) -> Vec<Instruction> {
        let mut ret = vec![];
        for instr in program {
            // NO-OPS
            if (instr.is_add() && instr.get_literal_b() == Some(0))
                || (instr.is_mul() && instr.get_literal_b() == Some(1))
                || (instr.is_div() && instr.get_literal_b() == Some(1))
                || (instr.is_mod() && instr.get_literal_b() == Some(1))
            {
                // Skip entirely
            } else if instr.is_mul() && instr.get_literal_b() == Some(0) {
                ret.push(Instruction::Set(instr.a(), Variable::Literal(0)));
            } else {
                ret.push(*instr);
            }
        }
        ret
    }
    fn read_program(str: &str) -> Vec<Instruction> {
        str.split("\n")
            .filter(|s| !s.is_empty())
            .map(Instruction::parse)
            .collect::<Vec<Instruction>>()
    }

    fn get(&self, var: Variable) -> Word {
        match var {
            Variable::Literal(v) => v,
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn set(&mut self, var: Variable, value: Word) {
        match var {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
            _ => panic!(),
        }
    }
}
impl Display for ALU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let us = format!("ALU: w={} x={} y={} z={}", self.w, self.x, self.y, self.z);
        f.write_str(&us)
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Variable {
    Literal(Word),
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn parse(raw: &str) -> Self {
        use Variable::*;
        match raw {
            "w" => W,
            "x" => X,
            "y" => Y,
            "z" => Z,
            _ => Literal(raw.parse::<Word>().expect("Variable fuckup")),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Literal(v) => v.fmt(f),
            Variable::W => f.write_str("w"),
            Variable::X => f.write_str("x"),
            Variable::Y => f.write_str("y"),
            Variable::Z => f.write_str("z"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Variable),
    Mul(Variable, Variable),
    Div(Variable, Variable),
    Mod(Variable, Variable),
    Eql(Variable, Variable),
    Set(Variable, Variable),
}

impl Instruction {
    fn parse(raw: &str) -> Self {
        use Instruction::*;
        let parts = raw
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        match parts[0].as_str() {
            "inp" => Inp(Variable::parse(&parts[1])),
            "add" => Add(Variable::parse(&parts[1]), Variable::parse(&parts[2])),
            "mul" => Mul(Variable::parse(&parts[1]), Variable::parse(&parts[2])),
            "div" => Div(Variable::parse(&parts[1]), Variable::parse(&parts[2])),
            "mod" => Mod(Variable::parse(&parts[1]), Variable::parse(&parts[2])),
            "eql" => Eql(Variable::parse(&parts[1]), Variable::parse(&parts[2])),
            _ => panic!("Input fuckup"),
        }
    }

    fn run(&self, alu: &mut ALU) {
        use Instruction::*;
        match self {
            Inp(a) => {
                // println!("{alu}");
                let value = alu.input.pop().unwrap();
                alu.set(*a, value)
            }
            Add(a, b) => alu.set(*a, alu.get(*a) + alu.get(*b)),
            Mul(a, b) => alu.set(*a, alu.get(*a) * alu.get(*b)),
            Div(a, b) => alu.set(*a, alu.get(*a) / alu.get(*b)),
            Mod(a, b) => alu.set(*a, alu.get(*a) % alu.get(*b)),
            Eql(a, b) => alu.set(*a, if alu.get(*a) == alu.get(*b) { 1 } else { 0 }),
            Set(a, b) => alu.set(*a, alu.get(*b)),
        }
        //   println!("{alu}");
    }

    /// Returns `true` if the instruction is [`Add`].
    ///
    /// [`Add`]: Instruction::Add
    #[must_use]
    fn is_add(&self) -> bool {
        matches!(self, Self::Add(..))
    }

    /// Returns `true` if the instruction is [`Mul`].
    ///
    /// [`Mul`]: Instruction::Mul
    #[must_use]
    fn is_mul(&self) -> bool {
        matches!(self, Self::Mul(..))
    }

    /// Returns `true` if the instruction is [`Div`].
    ///
    /// [`Div`]: Instruction::Div
    #[must_use]
    fn is_div(&self) -> bool {
        matches!(self, Self::Div(..))
    }

    /// Returns `true` if the instruction is [`Mod`].
    ///
    /// [`Mod`]: Instruction::Mod
    #[must_use]
    fn is_mod(&self) -> bool {
        matches!(self, Self::Mod(..))
    }

    /// Returns `true` if the instruction is [`Eql`].
    ///
    /// [`Eql`]: Instruction::Eql
    #[must_use]
    fn is_eql(&self) -> bool {
        matches!(self, Self::Eql(..))
    }

    fn a(&self) -> Variable {
        match self {
            Instruction::Inp(a) => *a,
            Instruction::Add(a, _) => *a,
            Instruction::Mul(a, _) => *a,
            Instruction::Div(a, _) => *a,
            Instruction::Mod(a, _) => *a,
            Instruction::Eql(a, _) => *a,
            Instruction::Set(a, _) => *a,
        }
    }

    fn get_literal_b(&self) -> Option<Word> {
        use Instruction::*;
        use Variable::Literal;
        match self {
            Add(_, Literal(b)) => Some(*b),
            Mul(_, Literal(b)) => Some(*b),
            Div(_, Literal(b)) => Some(*b),
            Mod(_, Literal(b)) => Some(*b),
            Eql(_, Literal(b)) => Some(*b),
            _ => None,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(a) => write!(f, "inp {a}"),
            Instruction::Add(a, b) => write!(f, "add {a} {b}"),
            Instruction::Mul(a, b) => write!(f, "mul {a} {b}"),
            Instruction::Div(a, b) => write!(f, "div {a} {b}"),
            Instruction::Mod(a, b) => write!(f, "mod {a} {b}"),
            Instruction::Eql(a, b) => write!(f, "eql {a} {b}"),
            Instruction::Set(a, b) => write!(f, "set {a} {b}"),
        }
    }
}

fn monad(secrets: &[(Word, Word, Word)], sn: &[Word]) -> Word {
    let mut z: Word = 0;
    for i in 0..14 {
        let mut w = sn[i];
        let k1 = secrets[i].0; // Either 1 or 26.
        let k2 = secrets[i].1; // A non-null integer, negative iff k1 == 26, greater than 10 otherwise.
        let k3 = secrets[i].2; // An integer, zero or positive.

        let mut x = z % 26 + k2;
        z /= k1;

        // When k1 == 26, k2 is negative, so if z %26== -k2 then x will be zero.
        if k1 == 26 && x > 0 {
            println!("w should be {x} (forcing)");
            w = x;
        } else if k1 == 26 {
            println!("No legal value for w;")
        }
        x = if x == w { 0 } else { 1 };
        let mut y = 25 * x + 1;
        z *= y;

        y = (w + k3) * x;

        z += y;
        // println!("w: {w}, k1: {k1:<3}, k2: {k2:<3}, k3: {k3:<3} => {z}");
    }
    z
}

// Given seven digits of a serial number, returns a complete SN or the index of the first invalid digit.
fn monad_guesser(secrets: &[(Word, Word, Word)], base: &[Word]) -> Result<Vec<Word>, usize> {
    let mut z: Word = 0;
    let mut base_idx = 0;
    let mut ret = vec![];
    for (k1, k2, k3) in secrets.iter().copied() {
        let mut x = z % 26 + k2;
        z /= k1;

        let w;
        // When k1 == 26, k2 is negative, so if z %26== -k2 then x will be zero.
        if k1 == 1 {
            w = base[base_idx];
            base_idx += 1
        } else if k1 == 26 && x > 0 && x < 10 {
            w = x;
        } else {
            return Err(base_idx);
        }
        ret.push(w);
        x = if x == w { 0 } else { 1 };
        let mut y = 25 * x + 1;
        z *= y;

        y = (w + k3) * x;

        z += y;
        // println!("w: {w}, k1: {k1:<3}, k2: {k2:<3}, k3: {k3:<3} => {z}");
    }
    Ok(ret)
}

fn main() {
    let program = ALU::read_program(&std::fs::read_to_string("inputs/24.txt").unwrap());

    let mut secrets = vec![];
    for idx in 0..14 {
        let a = program[idx * 18 + 4].get_literal_b().unwrap();
        let b = program[idx * 18 + 5].get_literal_b().unwrap();
        let c = program[idx * 18 + 15].get_literal_b().unwrap();
        secrets.push((a, b, c));
    }
    print!("Template: ");
    for (k1, _, _) in &secrets {
        if *k1 == 1 {
            print!(" _ ");
        } else {
            print!(" X ");
        }
    }
    println!("[_ = free X = forced]");

    // let mut base_sn: Vec<Word> = vec![9; 14];
    let mut part2 = true;
    let mut part1 = String::new();
    for grp in 0..9_i64.pow(7) {
        let mut base = vec![];
        for pow in 0..7 {
            // let a = ((grp1 / 81) % 9) + 1;
            // let b = ((grp1 / 9) % 9) + 1;
            // let c = (grp1 % 9) + 1;
            base.push(1 + (grp / 9_i64.pow(6 - pow) % 9));
        }
        let result = monad_guesser(&secrets, &base);
        if let Ok(ok) = result {
            let ok = ok.iter().map(|d| format!("{d}")).collect::<String>();
            if part2 {
                println!("Part 2: {ok}");
                part2 = false
            } else {
                part1 = ok;
            }
        }
    }
    println!("Part 1: {part1}");
}
