fn main() {
    let input = include_str!("../data/2015-06.txt");
    println!("lights: {}", part1(input));
    println!("lights: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let adjust = |act, light: &mut u16| {
        match act {
            TurnOn => *light = 1,
            TurnOff => *light = 0,
            Toggle => *light ^= 1
        }
    };
    lights(input, adjust)
}

fn part2(input: &str) -> u32 {
    let adjust = |act, light: &mut u16| {
        match act {
            TurnOn => *light += 1,
            TurnOff => *light = light.saturating_sub(1),
            Toggle => *light += 2
        }
    };
    lights(input, adjust)
}

fn lights(input: &str, adjust: fn(Action, &mut u16)) -> u32 {
    let mut lights = [[0u16; 1_000]; 1_000];
    for line in input.lines() {
        // parser approach 1: zero-allocation, single-pass
        let mut it = line.chars().enumerate();

        let (act, skips) = match it.nth(6) {
            Some((_, 'n')) => (TurnOn, 1),
            Some((_, 'f')) => (TurnOff, 2),
            _ => (Toggle, 0)
        };

        let mut i = 7;

        let mut parse_next_num = |skips| {
            if skips > 0 {
                it.nth(skips);
                i += skips;
            }
            while let Some((j, c)) = it.next() {
                if !c.is_numeric() {
                    let num = line[i..j].parse().unwrap();
                    i = j + 1;
                    return num;
                }
            }
            line[i..].parse().unwrap()
        };

        let x1 = parse_next_num(skips);
        let y1 = parse_next_num(0);
        let x2 = parse_next_num(8);
        let y2 = parse_next_num(0);

        // parser approach 2: zero-allocation, slightly-more-than-single-pass
        /*let act = if line.starts_with("turn on") {
            TurnOn
        } else if line.starts_with("turn off") {
            TurnOff
        } else {
            Toggle
        };

        let mut parts = line.split(' ');
        let (mut c1, mut c2) = match act {
            TurnOn | TurnOff => (
                parts.nth(2).unwrap().split(','),
                parts.nth(1).unwrap().split(',')
            ),
            Toggle => (
                parts.nth(1).unwrap().split(','),
                parts.nth(1).unwrap().split(',')
            )
        };
        let (x1, y1, x2, y2) = (
            c1.nth(0).unwrap().parse().unwrap(),
            c1.nth(0).unwrap().parse().unwrap(),
            c2.nth(0).unwrap().parse().unwrap(),
            c2.nth(0).unwrap().parse().unwrap(),
        );*/

        for row in &mut lights[y1..=y2] {
            for light in &mut row[x1..=x2] {
                adjust(act, light);
            }
        }
    }
    let mut on = 0;
    for row in &lights {
        for light in row {
            on += *light as u32;
        }
    }
    on
}

use Action::*;
#[derive(Copy, Clone)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

#[test]
fn test1() {
    assert_eq!(part1("turn on 0,0 through 999,999"), 1_000_000);
    assert_eq!(part1("toggle 0,0 through 999,0"), 1_000);
    assert_eq!(part1("turn off 499,499 through 500,500"), 0);
}

#[test]
fn test2() {
    assert_eq!(part2("turn on 0,0 through 0,0"), 1);
    assert_eq!(part2("toggle 0,0 through 999,999"), 2_000_000);
}
