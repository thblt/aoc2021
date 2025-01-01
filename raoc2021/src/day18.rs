#[derive(Debug, Eq, Copy, Clone, PartialEq, PartialOrd, Ord)]
enum Token {
    LParen,
    Comma,
    RParen,
    Number(u64),
}

impl Token {
    fn parse(s: &str) -> Vec<Token> {
        let mut ret: Vec<Token> = vec![];
        for c in s.chars() {
            match c {
                c if c.is_ascii_digit() => {
                    if ret.is_empty() || !ret.last().unwrap().is_number() {
                        ret.push(Token::Number(0))
                    }
                    *ret.last_mut().unwrap().number_mut() *= 10;
                    *ret.last_mut().unwrap().number_mut() += c.to_digit(10).unwrap() as u64;
                }
                '[' => ret.push(Token::LParen),
                ']' => ret.push(Token::RParen),
                ',' => ret.push(Token::Comma),
                _ => panic!(),
            }
        }
        ret
    }

    /// Returns `true` if the token is [`Number`].
    ///
    /// [`Number`]: Token::Number
    #[must_use]
    fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    fn number_mut(&mut self) -> &mut u64 {
        if let Token::Number(ret) = self {
            ret
        } else {
            panic!()
        }
    }
    fn number(&self) -> u64 {
        if let Token::Number(ret) = self {
            *ret
        } else {
            panic!()
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LParen => f.write_str("["),
            Token::Comma => f.write_str(","),
            Token::RParen => f.write_str("]"),
            Token::Number(n) => n.fmt(f),
        }
    }
}

fn split(seq: &[Token]) -> Option<Vec<Token>> {
    let mut ret = vec![];
    let mut done = false;
    for t in seq.iter() {
        if !done && t.is_number() && t.number() >= 10 {
            let n = t.number();
            ret.push(Token::LParen);
            ret.push(Token::Number(n / 2));
            ret.push(Token::Comma);
            ret.push(Token::Number(n / 2 + n % 2));
            ret.push(Token::RParen);
            done = true;
        } else {
            ret.push(*t)
        }
    }
    if done {
        Some(ret)
    } else {
        None
    }
}

#[test]
fn test_split() {
    assert!(split(&Token::parse("10")) == Some(Token::parse("[5,5]")));
    assert!(split(&Token::parse("11")) == Some(Token::parse("[5,6]")));
    assert!(split(&Token::parse("[11,12]")) == Some(Token::parse("[[5,6],12]")));
}

fn explode(seq: &[Token]) -> Option<Vec<Token>> {
    let mut left: Option<usize> = None;
    let mut right: Option<usize> = None;
    let mut depth = 0;
    let mut start: Option<usize> = None;
    let mut counter = 0;
    for (idx, token) in seq.iter().enumerate() {
        match token {
            Token::Number(_) if start.is_none() => left = Some(idx),
            Token::Number(_) if right.is_none() && start.is_some() && counter == 2 => {
                right = Some(idx)
            }
            Token::Number(_) if right.is_none() && start.is_some() => counter += 1,
            Token::LParen
                if depth == 4
                    && start.is_none()
                    && seq[idx + 1].is_number()
                    && seq[idx + 3].is_number() =>
            {
                start = Some(idx)
            }
            Token::LParen => depth += 1,
            Token::RParen => depth -= 1,
            _ => {}
        }
    }
    if let Some(start) = start {
        let mut ret = seq.to_vec();
        if let Some(left) = left {
            *ret[left].number_mut() += ret[start + 1].number();
        }
        if let Some(right) = right {
            *ret[right].number_mut() += ret[start + 3].number();
        }

        let ret = ret
            .into_iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if i == start {
                    Some(Token::Number(0))
                } else if i > start && i <= start + 4 {
                    None
                } else {
                    Some(t)
                }
            })
            .collect::<Vec<Token>>();
        Some(ret)
    } else {
        None
    }
}

#[test]
fn test_explode() {
    assert!(
        explode(&Token::parse("[[[[[9,8],1],2],3],4]")) == Some(Token::parse("[[[[0,9],2],3],4]"))
    );
    assert!(
        explode(&Token::parse("[7,[6,[5,[4,[3,2]]]]]")) == Some(Token::parse("[7,[6,[5,[7,0]]]]"))
    );
    assert!(
        explode(&Token::parse("[[6,[5,[4,[3,2]]]],1]")) == Some(Token::parse("[[6,[5,[7,0]]],3]"))
    );
    assert!(
        explode(&Token::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"))
            == Some(Token::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );

    assert!(
        explode(&Token::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
            == Some(Token::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
    );
}

fn reduce(seq: &[Token]) -> Vec<Token> {
    let mut seq = seq.to_vec();
    let mut fixed_point = false;
    while !fixed_point {
        fixed_point = true;
        if let Some(exploded) = explode(&seq) {
            seq = exploded;
            fixed_point = false;
            continue;
        }
        if let Some(splitted) = split(&seq) {
            seq = splitted;
            fixed_point = false;
        }
    }
    seq
}

fn add(lhs: &[Token], rhs: &[Token]) -> Vec<Token> {
    use std::iter::once;
    once(&Token::LParen)
        .chain(lhs.iter())
        .chain(once(&Token::Comma))
        .chain(rhs.iter())
        .chain(once(&Token::RParen))
        .copied()
        .collect::<Vec<Token>>()
}

fn add_reduce(lhs: &[Token], rhs: &[Token]) -> Vec<Token> {
    reduce(&add(lhs, rhs))
}

fn magnitude(seq: &[Token]) -> u64 {
    if seq.len() == 1 {
        return seq[0].number();
    }
    let mut depth = 0;
    let mut split = 0;
    for (idx, t) in seq.iter().enumerate() {
        match t {
            Token::LParen => depth += 1,
            Token::RParen => depth -= 1,
            Token::Comma if depth == 1 => {
                split = idx;
                break;
            }
            _ => {}
        };
    }
    3 * magnitude(&seq[1..split]) + 2 * magnitude(&seq[split + 1..seq.len() - 1])
}

#[test]
fn test_magnitude() {
    assert!(magnitude(&Token::parse("[[1,2],[[3,4],5]]")) == 143);
    assert!(magnitude(&Token::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")) == 1384);
    assert!(magnitude(&Token::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")) == 445);
    assert!(magnitude(&Token::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")) == 791);
    assert!(magnitude(&Token::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")) == 1137);
    assert!(
        magnitude(&Token::parse(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )) == 3488
    );
}

fn main() {
    let mut acc: Option<Vec<Token>> = None;
    let homework = lib::read_lines("inputs/18.txt")
        .unwrap()
        .map(|l| Token::parse(&l.unwrap()))
        .collect::<Vec<Vec<Token>>>();
    for line in &homework {
        if let Some(acc) = &mut acc {
            *acc = add_reduce(acc, line);
        } else {
            acc = Some(line.clone())
        }
    }
    println!("Part 1: {}", magnitude(&acc.unwrap()));

    let mut part2 = 0;
    for lhs in &homework {
        for rhs in &homework {
            if lhs == rhs {
                continue;
            }
            part2 = std::cmp::max(magnitude(&add_reduce(lhs, rhs)), part2);
        }
    }
    println!("Part 2: {part2}");
}
