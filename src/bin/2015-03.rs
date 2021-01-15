const INPUT: &str = include_str!("../data/2015-03.txt");

use std::collections::HashSet;

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


fn part1() {
    let mut santa = Santa::new();
    for m in INPUT.chars() {
        santa.go(m);
    }
    println!("houses: {}", santa.houses.len());
}

fn part2() {
    let mut santa = Santa::new();
    let mut robosanta = Santa::new();
    let mut direct_santa = true;

    for m in INPUT.chars() {
        if direct_santa {
            santa.go(m);
        } else {
            robosanta.go(m);
        }
        direct_santa = !direct_santa;
    }

    println!("houses: {}", santa.houses.union(&robosanta.houses).count());
}

fn main() {
    part1();
    part2();
}
