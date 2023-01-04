use itertools::Itertools;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};
use std::fmt;

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

struct Piles(Vec<Vec<Crate>>);

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }

    fn apply_p2(&mut self, ins: Instruction) {
        for krate in (0..ins.quantity)
            .map(|_| self.0[ins.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.dst].push(krate);
        }
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let first_c = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_c)(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_number, |n| n - 1)(input)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

pub fn p1() {
    let mut lines = include_str!("../input/day5.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, maybe_crate)| maybe_crate)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));

    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    for &ins in &instructions {
        piles.apply_p2(ins);
    }

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}
