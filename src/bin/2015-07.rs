use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/2015-07.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u16 {
    let mut wires = parse(input);
    calc(&mut wires, "a")
}

fn part2(input: &str) -> u16 {
    let mut wires1 = parse(input);
    let mut wires2 = wires1.clone();
    let a1 = calc(&mut wires1, "a");
    wires2.insert("b", Computed(a1));
    calc(&mut wires2, "a")
}

fn parse(input: &str) -> HashMap<&str, Op> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let op = match parts[1] {
            "AND" => And(parts[0], parts[2]),
            "OR" => Or(parts[0], parts[2]),
            "LSHIFT" => LShift(parts[0], parts[2]),
            "RSHIFT" => RShift(parts[0], parts[2]),
            "->" => Assign(parts[0]),
            _ => Not(parts[1])
        };
        wires.insert(parts[parts.len()-1], op);
    }
    wires
}

fn calc<'a>(wires: &mut HashMap<&'a str, Op<'a>>, id: &'a str) -> u16 {
    if let Ok(num) = id.parse() {
        return num
    }
    let op = wires[id];
    let res = match op {
        Computed(n) => return n,
        Assign(x) => calc(wires, x),
        Not(x) => !calc(wires, x),
        And(x, y) => calc(wires, x) & calc(wires, y),
        Or(x, y) => calc(wires, x) | calc(wires, y),
        LShift(x, y) => calc(wires, x) << calc(wires, y),
        RShift(x, y) => calc(wires, x) >> calc(wires, y)
    };
    //println!("memoizing {}={:?} into {}=Computed({})", id, op, id, res);
    wires.insert(id, Computed(res));
    res
}

use Op::*;
#[derive(Copy, Clone, Debug)]
enum Op<'a> {
    Computed(u16),
    Assign(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, &'a str),
    RShift(&'a str, &'a str)
}

#[test]
fn test1() {
    aoc::test(part1, [
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
