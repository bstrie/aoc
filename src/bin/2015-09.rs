#![feature(array_windows)]

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet}
};

fn main() {
    let input = include_str!("../data/2015-09.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
	let mut places = Places::new(input, min);
	places.permute(i32::MAX, places.names.len())
}

fn part2(input: &str) -> i32 {
	let mut places = Places::new(input, max);
	places.permute(0, places.names.len())
}

struct Places<'a, F> {
	names: Vec<&'a str>,
	distances: HashMap<(&'a str, &'a str), i32>,
    compare: F
}

impl<'a, F: Fn(i32,i32)->i32> Places<'a, F> {
	fn new(input: &str, compare: F) -> Places<F> {
		let mut names = HashSet::new();
		let mut distances = HashMap::new();
		for line in input.lines() {
			let parts: Vec<_> = line.split(' ').collect();
			names.insert(parts[0]);
			names.insert(parts[2]);
			distances.insert(
				if parts[0] < parts[2] {
					(parts[0], parts[2])
				} else {
					(parts[2], parts[0])
				},
				parts[4].parse().unwrap()
			);
		}
		Places {
			names: names.into_iter().collect(),
			distances,
			compare
		}
	}

	fn permute(&mut self, mut dist: i32, j: usize) -> i32 {
		if j == 1 {
			let mut total = 0;
			for &[p1, p2] in self.names.array_windows() {
                total += self.distances[&
                    if p1 < p2 {
					    (p1, p2)
                    } else {
					    (p2, p1)
                    }
                ];
			}
			return total;
		}

		for i in 0..j {
            let newdist = self.permute(dist, j-1);
			dist = (self.compare)(dist, newdist);

			if j % 2 == 0 {
				self.names.swap(i, j-1);
			} else {
				self.names.swap(0, j-1);
			}
		}

        dist
	}
}

#[test]
fn test1() {
    let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    assert_eq!(part1(input), 605);
}

#[test]
fn test2() {
    let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    assert_eq!(part2(input), 982);
}
