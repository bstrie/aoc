use regex::Regex;

fn main() {
    let input = include_str!("../data/2015-06.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let adjust = |act: &str, light: &mut u16| {
        match act {
            "turn on" => *light = 1,
            "turn off" => *light = 0,
            _ => *light ^= 1
        }
    };
    lights(input, adjust)
}

fn part2(input: &str) -> u32 {
    let adjust = |act: &str, light: &mut u16| {
        match act {
            "turn on" => *light += 1,
            "turn off" => *light = light.saturating_sub(1),
            _ => *light += 2
        }
    };
    lights(input, adjust)
}

fn lights(input: &str, adjust: fn(&str, &mut u16)) -> u32 {
    let mut lights = [[0u16; 1_000]; 1_000];
    let pattern = r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)";
    for line in Regex::new(pattern).unwrap().captures_iter(input) {
        let x1 = line[2].parse().unwrap();
        let y1 = line[3].parse().unwrap();
        let x2 = line[4].parse().unwrap();
        let y2 = line[5].parse().unwrap();

        for row in &mut lights[y1..=y2] {
            for light in &mut row[x1..=x2] {
                adjust(&line[1], light);
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

#[test]
fn test1() {
    aoc::test(part1, [
        ("turn on 0,0 through 999,999", 1_000_000),
        ("toggle 0,0 through 999,0", 1_000),
        ("turn off 499,499 through 500,500", 0)
    ])
}

#[test]
fn test2() {
    aoc::test(part2, [
        ("turn on 0,0 through 0,0", 1),
        ("toggle 0,0 through 999,999", 2_000_000)
    ])
}
