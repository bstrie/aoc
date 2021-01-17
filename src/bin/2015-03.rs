use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/2015-03.txt");
    println!("houses: {}", part1(input));
    println!("houses: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut santa = Santa::new();
    for m in input.chars() {
        santa.go(m);
    }
    santa.houses.len()
}

fn part2(input: &str) -> usize {
    let mut santa = Santa::new();
    let mut robosanta = Santa::new();
    let mut direct_santa = true;

    for m in input.chars() {
        if direct_santa {
            santa.go(m);
        } else {
            robosanta.go(m);
        }
        direct_santa = !direct_santa;
    }

    santa.houses.union(&robosanta.houses).count()
}

struct Santa {
    x: i32,
    y: i32,
    houses: HashSet<(i32, i32)>
}

impl Santa {
    fn new() -> Santa {
        let mut h = HashSet::new();
        h.insert((0, 0));
        Santa { x: 0, y: 0, houses: h }
    }

    fn go(&mut self, dir: char) {
        match dir {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => ()
        }
        self.houses.insert((self.x, self.y));
    }
}

#[test]
fn test1() {
    aoc::test(part1, &[
        (">", 2),
        ("^>v<", 4),
        ("^v^v^v^v^v", 2)
    ]);
}

#[test]
fn test2() {
    aoc::test(part2, &[
        ("^v", 3),
        ("^>v<", 3),
        ("^v^v^v^v^v", 11)
    ]);
}
