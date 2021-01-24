use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/2015-09.txt");
    println!("distance: {}", part1(input));
    println!("distance: {}", part2(input));
}

fn part1(input: &str) -> i32 {
	let mut places = Places::new(input, true);
	places.permute(places.names.len());
	places.travel_distance
}

fn part2(input: &str) -> i32 {
	let mut places = Places::new(input, false);
	places.permute(places.names.len());
	places.travel_distance
}

struct Places<'a> {
	names: Vec<&'a str>,
	distances: HashMap<(&'a str, &'a str), i32>,
	find_least: bool,
	travel_distance: i32
}

impl<'a> Places<'a> {
	fn new(input: &str, find_least: bool) -> Places {
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
			find_least,
			travel_distance: if find_least { i32::MAX } else { 0 }
		}
	}

	fn permute(&mut self, j: usize) {
		if j == 1 {
			let mut total = 0;
			for pair in self.names.windows(2) {
                total += self.distances[&
                    if pair[0] < pair[1] {
					    (pair[0], pair[1])
                    } else {
					    (pair[1], pair[0])
                    }
                ];
			}
			if (total < self.travel_distance) == self.find_least {
				self.travel_distance = total;
			}
			return;
		}

		for i in 0..j {
			self.permute(j-1);
			if j % 2 == 0 {
				self.names.swap(i, j-1);
			} else {
				self.names.swap(0, j-1);
			}
		}
	}
}

#[test]
fn test1() {
    aoc::test(part1, &[
        ("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141", 605)
    ])
}

#[test]
fn test2() {
    aoc::test(part2, &[
        ("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141", 982)
    ])
}
