fn main() {
    let input = include_str!("../data/2015-05.txt");
    println!("nice: {}", part1(input));
    println!("nicer: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input.lines().filter(|&s| is_nice(s)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|&s| is_nicer(s)).count()
}

fn is_nice(s: &str) -> bool {
    let mut prior = None;
    let mut vowels = 0;
    let mut has_doubles = false;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowels += 1;
        }
        if let Some(pc) = prior {
            match (pc, c) {
                ('a', 'b') | ('c', 'd') |
                ('p', 'q') | ('x', 'y') => return false,
                (c1, c2) => if c1 == c2 { has_doubles = true }
            }
        }
        prior = Some(c);
    }
    has_doubles && vowels > 2
}

fn is_nicer(s: &str) -> bool {
    let sb = s.as_bytes();
    let mut has_double_pair = false;
    let mut has_sandwiched = false;
    for i in 0..sb.len() {
        if i < sb.len()-3 {
            for sw in sb[i+2..].windows(2) {
                if sb[i] == sw[0] && sb[i+1] == sw[1] {
                    has_double_pair = true;
                }
            }
        }
        if i < sb.len()-2 && sb[i] == sb[i+2] {
            has_sandwiched = true;
        }
    }
    has_double_pair && has_sandwiched
}

use aoc::Test;

#[test]
fn test1() {
    is_nice.test(&[
        ("ugknbfddgicrmopn", true),
        ("aaa", true),
        ("jchzalrnumimnmhp", false),
        ("haegwjzuvuyypxyu", false),
        ("dvszwmarrgswjxmb", false)
    ])
}

#[test]
fn test2() {
    is_nicer.test(&[
        ("qjhvhtzxzqqjkmpb", true),
        ("xxyxx", true),
        ("uurcxstgmygtbstg", false),
        ("ieodomkazucvgmuy", false)
    ])
}
