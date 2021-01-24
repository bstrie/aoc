use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/2015-07.txt");
    println!("a: {}", part1(input));
    println!("a: {}", part2(input));
}

fn part1(input: &str) -> u16 {
    let mut wires = parse(input);
    calc(&mut wires, String::from("a"))
}

fn part2(input: &str) -> u16 {
    let mut wires1 = parse(input);
    let mut wires2 = wires1.clone();
    let a = calc(&mut wires1, String::from("a"));
    wires2.insert(String::from("b"), Assign(Num(a)));
    calc(&mut wires2, String::from("a"))
}

fn parse(input: &str) -> HashMap<String, Op> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let parts: Vec<String> = line
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        let (id, op) = match parts.len() {
            3 => (parts[2].clone(), Assign(parts[0].clone().into())),
            4 => (parts[3].clone(), Not(parts[1].clone().into())),
            5 => (parts[4].clone(), match parts[1].as_str() {
                "AND" => And(parts[0].clone().into(), parts[2].clone().into()),
                "OR" => Or(parts[0].clone().into(), parts[2].clone().into()),
                "LSHIFT" => LShift(parts[0].clone().into(), parts[2].clone().into()),
                "RSHIFT" => RShift(parts[0].clone().into(), parts[2].clone().into()),
                _ => unreachable!()
            }),
            _ => unreachable!()
        };
        wires.insert(id, op);
    }
    wires
}

fn calc(wires: &mut HashMap<String, Op>, id: String) -> u16 {
    let op = wires[&id].clone();
    let num = match op.clone() {
        Assign(v) => match v {
            Num(n) => return n, // skip memoization
            Id(i) => calc(wires, i)
        }
        Not(v) => match v {
            Num(n) => !n,
            Id(i) => !calc(wires, i)
        }
        And(v, w) => match (v, w) {
            (Num(n), Num(m)) => n & m,
            (Num(n), Id(i)) |
            (Id(i), Num(n)) => n & calc(wires, i),
            (Id(i), Id(j)) => calc(wires, i) & calc(wires, j)
        }
        Or(v, w) => match (v, w) {
            (Num(n), Num(m)) => n | m,
            (Num(n), Id(i)) |
            (Id(i), Num(n)) => n | calc(wires, i),
            (Id(i), Id(j)) => calc(wires, i) | calc(wires, j)
        }
        LShift(v, w) => match (v, w) {
            (Num(n), Num(m)) => n << m,
            (Num(n), Id(i)) => n << calc(wires, i),
            (Id(i), Num(n)) => calc(wires, i) << n,
            (Id(i), Id(j)) => calc(wires, i) << calc(wires, j)
        }
        RShift(v, w) => match (v, w) {
            (Num(n), Num(m)) => n >> m,
            (Num(n), Id(i)) => n >> calc(wires, i),
            (Id(i), Num(n)) => calc(wires, i) >> n,
            (Id(i), Id(j)) => calc(wires, i) >> calc(wires, j)
        }
    };
    //println!("memoizing {}={:?} into {}=Assign(Num({}))", id, op, id, num);
    wires.insert(id, Assign(Num(num)));
    num
}


use Val::*;
#[derive(Clone, Debug)]
enum Val {
    Num(u16),
    Id(String)
}

impl std::convert::From<String> for Val {
    fn from(val: String) -> Val {
        if val.chars().all(char::is_numeric) {
            Num(val.parse().unwrap())
        } else {
            Id(val)
        }
    }
}

use Op::*;
#[derive(Clone, Debug)]
enum Op {
    Assign(Val),
    Not(Val),
    And(Val, Val),
    Or(Val, Val),
    LShift(Val, Val),
    RShift(Val, Val)
}

use aoc::Test;

#[test]
fn test1() {
    part1.test(&[
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
    ])
}
