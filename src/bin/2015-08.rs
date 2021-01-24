fn main() {
    let input = include_str!("../data/2015-08.txt");
    println!("difference: {}", part1(input));
    println!("difference: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        count += line.len();
        let mut it = line.chars();
        while let Some(c) = it.next() {
            if c == '"' {
                continue;
            } else if c == '\\' {
                if let Some(c2) = it.next() {
                    if c2 == 'x' {
                        it.next();
                        it.next();
                    }
                }
            }
            count -= 1;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        count += 2;
        for c in line.chars() {
            if c == '"' || c == '\\' {
                count += 1;
            }
            count += 1
        }
        count -= line.len();
    }
    count
}

use aoc::Test;

#[test]
fn test1() {
    part1.test(&[
        (r#""""#, 2),
        (r#""abc""#, 2),
        (r#""aaa\"aaa""#, 3),
        (r#""\x27""#, 5)
    ])
}

#[test]
fn test2() {
    part2.test(&[
        (r#""""#, 4),
        (r#""abc""#, 4),
        (r#""aaa\"aaa""#, 6),
        (r#""\x27""#, 5)
    ])
}
