const INPUT: &str = include_str!("../data/2015-01.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut floor = 0;
    for c in INPUT.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    println!("floor: {}", floor);
}

fn part2() {
    let mut floor = 0;
    for (i, c) in INPUT.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor < 0 {
            println!("position: {}", i+1);
            break;
        }
    }
}
