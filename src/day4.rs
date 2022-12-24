use std::ops::RangeInclusive;
use itertools::Itertools;

pub fn p1() {
    let input = include_str!("../input/day4.txt");
    let count: u32 = input.lines()
        .into_iter()
        .map(|x| parse_ranges(x))
        .map(|(a, b)| overlap((a, b)))
        .filter(|b| *b == true)
        .count() as u32;
    println!("Count is: {}", count);
}

pub fn p2() {
    let input = include_str!("../input/day4.txt");
    let count: u32 = input.lines()
        .into_iter()
        .map(|x| parse_ranges(x))
        .map(|(a, b)| partial_overlap((a, b)))
        .filter(|b| *b == true)
        .count() as u32;
    println!("Count is: {}", count);
}

fn parse_ranges(input: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    input.split(",")
        .map(|range| {
            range.split("-")
                .map(|num| num.parse().expect("Should be a u32"))
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .map(|(a, b)| RangeInclusive::new(a, b) )
        .collect_tuple::<(_, _)>()
        .expect("Each pair should have a range")
}

fn overlap((a, b): (RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    a.contains_range(&b) && b.contains_range(&a)
}

fn partial_overlap((a, b): (RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    a.contains_some_of_range(&b) || b.contains_some_of_range(&a)
}

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_some_of_range(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where T:
    PartialOrd
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn contains_some_of_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}