use std::cmp::max;

fn main() {
    let input = include_str!("../data/2015-15.txt");
    println!("Part 1: {}", part1(input, 100));
}

fn part1(input: &str, total_spoons: i64) -> i64 {
    let mut ingredients = Ingredients::new(input, total_spoons);
    ingredients.evaluate()
}

struct Ingredients {
    capacity: Vec<i64>,
    durability: Vec<i64>,
    flavor: Vec<i64>,
    texture: Vec<i64>,
    calories: Vec<i64>,
    spoons: Vec<i64>,
    num: usize,
    total_spoons: i64,
}

impl Ingredients {
    fn new(input: &str, total_spoons: i64) -> Ingredients {
        let mut num = 0;
        let mut capacity = Vec::new();
        let mut durability = Vec::new();
        let mut flavor = Vec::new();
        let mut texture = Vec::new();
        let mut calories = Vec::new();
        for line in input.lines() {
            let parts: Vec<_> = line.split(' ').collect();
            capacity.push(parts[2].trim_end_matches(',').parse().unwrap());
            durability.push(parts[4].trim_end_matches(',').parse().unwrap());
            flavor.push(parts[6].trim_end_matches(',').parse().unwrap());
            texture.push(parts[8].trim_end_matches(',').parse().unwrap());
            calories.push(parts[10].parse().unwrap());
            num += 1;
        }
        let even_spoons = total_spoons / num as i64;
        let mut spoons = vec![even_spoons; num];
        for i in 0..(total_spoons as usize % num) {
            spoons[i] += 1;
        }
        Ingredients {
            capacity,
            durability,
            flavor,
            texture,
            calories,
            spoons,
            num,
            total_spoons,
        }
    }

    fn evaluate(&mut self) -> i64 {
        let mut base_score = self.score(&self.spoons);
        loop {
            let mut best_mutant_score = 0;
            let mut best_mutant_spoons = Vec::new();
            for i in 0..self.num {
                let bigger_spoon = self.spoons[i] + 1;
                if bigger_spoon <= self.total_spoons {
                    for j in 0..self.num {
                        if i != j {
                            let smaller_spoon = self.spoons[j] - 1;
                            if smaller_spoon > 0 {
                                let mut mutant_spoons = self.spoons.clone();
                                mutant_spoons[i] = bigger_spoon;
                                mutant_spoons[j] = smaller_spoon;
                                let mutant_score = self.score(&mutant_spoons);
                                if mutant_score > best_mutant_score {
                                    best_mutant_score = mutant_score;
                                    best_mutant_spoons = mutant_spoons;
                                }
                            }
                        }
                    }
                }
            }
            if best_mutant_score > base_score {
                base_score = best_mutant_score;
                self.spoons = best_mutant_spoons;
            } else {
                break base_score;
            }
        }
    }

    fn score(&self, spoons: &Vec<i64>) -> i64 {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        for i in 0..spoons.len() {
            capacity += self.capacity[i] * spoons[i];
            durability += self.durability[i] * spoons[i];
            flavor += self.flavor[i] * spoons[i];
            texture += self.texture[i] * spoons[i];
        }
        capacity = max(0, capacity);
        durability = max(0, durability);
        flavor = max(0, flavor);
        texture = max(0, texture);
        capacity * durability * flavor * texture
    }
}

#[test]
fn test1() {
    let input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    assert_eq!(part1(input, 2), 8);
    assert_eq!(part1(input, 100), 62842880);
}
