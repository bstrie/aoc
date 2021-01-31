use std::cmp::{max, min};

fn main() {
    let input = include_str!("../data/2015-14.txt");
    println!("Part 1: {}", part1(input, 2503));
    println!("Part 2: {}", part2(input, 2503));
}

fn part1(input: &str, secs: i32) -> i32 {
    let deers = parse(input);
    deers.iter().fold(0, |best, deer| {
        let cycles = secs / (deer.fly_max + deer.rest_max);
        let remaining_secs = secs % (deer.fly_max + deer.rest_max);

        let mut dist = cycles * (deer.speed * deer.fly_max);
        dist += deer.speed * min(deer.fly_max, remaining_secs);

        max(best, dist)
    })
}

fn part2(input: &str, secs: i32) -> i32 {
    let mut deers = parse(input);
    for _ in 0..secs {
        let best = deers.iter_mut().fold(0, |best, deer| {
            deer.status += 1;
            if deer.status <= deer.fly_max {
                deer.distance += deer.speed;
            }
            if deer.status == deer.fly_max + deer.rest_max {
                deer.status = 0;
            }
            max(best, deer.distance)
        });
        for deer in &mut deers {
            if deer.distance == best {
                deer.score += 1;
            }
        }
    }
    deers.iter().fold(0, |best, deer| max(best, deer.score))
}

struct Deer {
    speed: i32,
    fly_max: i32,
    rest_max: i32,
    status: i32,
    distance: i32,
    score: i32
}

fn parse(input: &str) -> Vec<Deer> {
    let mut deers = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        deers.push(Deer {
            speed: parts[3].parse().unwrap(),
            fly_max: parts[6].parse().unwrap(),
            rest_max: parts[13].parse().unwrap(),
            status: 0,
            distance: 0,
            score: 0
        });
    }
    deers
}

#[test]
fn test1() {
    let input = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(part1(input, 1000), 1120);
}

#[test]
fn test2() {
    let input = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(part2(input, 1000), 689);
}
