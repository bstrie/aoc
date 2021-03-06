fn main() {
    let input = include_str!("../data/2015-01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    let mut position = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor < 0 {
            position = i+1;
            break;
        }
    }
    position
}

#[test]
fn test1() {
    aoc::test(part1, [
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3)
    ])
}

#[test]
fn test2() {
    aoc::test(part2, [
        (")", 1),
        ("()())", 5)
    ])
}
