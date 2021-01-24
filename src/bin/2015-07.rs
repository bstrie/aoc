use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/2015-07.txt");
    println!("a: {}", part1(input));
    println!("a: {}", part2(input));
}

fn part1(input: &str) -> u16 {
    let mut wires = parse(input);
    calc(&mut wires, Id("a"))
}

fn part2(input: &str) -> u16 {
    let mut wires1 = parse(input);
    let mut wires2 = wires1.clone();
    let a1 = calc(&mut wires1, Id("a"));
    wires2.insert("b", Computed(a1));
    calc(&mut wires2, Id("a"))
}

fn parse(input: &str) -> HashMap<&str, Op> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let (id, op) = match parts.len() {
            3 => (parts[2], Assign(parts[0].into())),
            4 => (parts[3], Not(parts[1].into())),
            5 => (parts[4], match parts[1] {
                "AND" => And(parts[0].into(), parts[2].into()),
                "OR" => Or(parts[0].into(), parts[2].into()),
                "LSHIFT" => LShift(parts[0].into(), parts[2].into()),
                "RSHIFT" => RShift(parts[0].into(), parts[2].into()),
                _ => unreachable!()
            }),
            _ => unreachable!()
        };
        wires.insert(id, op);
    }
    wires
}

fn calc<'a>(wires: &mut HashMap<&'a str, Op<'a>>, val: Val<'a>) -> u16 {
    let id = match val {
        Num(n) => return n, // skip lookup
        Id(i) => i
    };
    let op = wires[id];
    let num = match op {
        Computed(n) => return n, // skip memoization
        Assign(v) => calc(wires, v),
        Not(v) => !calc(wires, v),
        And(v, w) => calc(wires, v) & calc(wires, w),
        Or(v, w) => calc(wires, v) | calc(wires, w),
        LShift(v, w) => calc(wires, v) << calc(wires, w),
        RShift(v, w) => calc(wires, v) >> calc(wires, w)
    };
    //println!("memoizing {}={:?} into {}=Computed({})", id, op, id, num);
    wires.insert(id, Computed(num));
    num
}

use Val::*;
#[derive(Copy, Clone, Debug)]
enum Val<'a> {
    Num(u16),
    Id(&'a str)
}

impl<'a> std::convert::From<&'a str> for Val<'a> {
    fn from(val: &str) -> Val {
        if val.chars().all(char::is_numeric) {
            Num(val.parse().unwrap())
        } else {
            Id(val)
        }
    }
}

use Op::*;
#[derive(Copy, Clone, Debug)]
enum Op<'a> {
    Computed(u16),
    Assign(Val<'a>),
    Not(Val<'a>),
    And(Val<'a>, Val<'a>),
    Or(Val<'a>, Val<'a>),
    LShift(Val<'a>, Val<'a>),
    RShift(Val<'a>, Val<'a>)
}

#[test]
fn test1() {
    aoc::test(part1, &[
        ("123 -> a", 123),
        ("NOT 0 -> a", 65535),
        ("1 AND 1 -> a", 1),
        ("0 OR 1 -> a", 1),
        ("1 LSHIFT 2 -> a", 4),
        ("4 RSHIFT 2 -> a", 1),
        ("b -> a\n2 -> b", 2),
        ("123 -> x\n456 -> y\nx AND y -> a", 72),
        ("123 -> x\n456 -> y\nx OR y -> a", 507),
        ("123 -> x\n456 -> y\nx LSHIFT 2 -> a", 492),
        ("123 -> x\n456 -> y\ny RSHIFT 2 -> a", 114),
        ("123 -> x\n456 -> y\nNOT x -> a", 65412),
    ]);
}
