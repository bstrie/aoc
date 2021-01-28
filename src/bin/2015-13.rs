use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/2015-13.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
	let mut people = People::new(input);
	people.permute(0, people.names.len())
}

fn part2(input: &str) -> i32 {
	let mut people = People::new(input);
    people.names.push("Me");
	people.permute(0, people.names.len())
}

struct People<'a> {
	names: Vec<&'a str>,
	prefs: HashMap<(&'a str, &'a str), i32>,
}

impl<'a> People<'a> {
	fn new(input: &str) -> People {
		let mut names = HashSet::new();
		let mut prefs = HashMap::new();
		for line in input.lines() {
			let parts: Vec<_> = line.split(' ').collect();
            let p1 = parts[0];
            let p2 = parts[10].strip_suffix(".").unwrap();
            let magnitude: i32 = parts[3].parse().unwrap();
			names.insert(p1);
			names.insert(p2);
			prefs.insert(
                (p1, p2),
                if parts[2] == "lose" {
                    -magnitude
                } else {
                    magnitude
                }
			);
		}
		People {
			names: names.into_iter().collect(),
			prefs,
		}
	}

	fn permute(&mut self, mut happ: i32, j: usize) -> i32 {
		if j == 1 {
			let mut total = 0;
            let mut it = self.names.iter().peekable();
            while let Some(&p1) = it.next() {
                let &&p2 = it.peek().unwrap_or(&&self.names[0]);
                total += self.prefs.get(&(p1, p2)).unwrap_or(&0);
                total += self.prefs.get(&(p2, p1)).unwrap_or(&0);
            }
			return total;
		}

		for i in 0..j {
			happ = happ.max(self.permute(happ, j-1));
			if j % 2 == 0 {
				self.names.swap(i, j-1);
			} else {
				self.names.swap(0, j-1);
			}
		}

        happ
	}
}

#[test]
fn test1() {
    let input = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
    assert_eq!(part1(input), 330);
}
