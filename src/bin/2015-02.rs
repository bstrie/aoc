fn main() {
    let input = include_str!("../data/2015-02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut total_area = 0;
    for line in input.lines() {
        let d: Vec<i32> = line
            .split('x')
            .map(|n| n.parse().unwrap())
            .collect();

        let faces = [
            d[0]*d[1],
            d[1]*d[2],
            d[2]*d[0]
        ];

        total_area += faces.iter().min().unwrap();
        total_area += 2 * faces.iter().sum::<i32>();
    }
    total_area
}

fn part2(input: &str) -> i32 {
    let mut total_length = 0;
    for line in input.lines() {
        let d: Vec<i32> = line
            .split('x')
            .map(|n| n.parse().unwrap())
            .collect();

        let (da, db) = if d[0] > d[1] && d[0] > d[2] {
            (d[1], d[2])
        } else if d[1] > d[2] {
            (d[0], d[2])
        } else {
            (d[0], d[1])
        };

        total_length += 2*da + 2*db;
        total_length += d[0] * d[1] * d[2];
    }
    total_length
}

#[test]
fn test1() {
    aoc::test(part1, [
        ("2x3x4", 58),
        ("1x1x10", 43)
    ])
}

#[test]
fn test2() {
    aoc::test(part2, [
        ("2x3x4", 34),
        ("1x1x10", 14)
    ])
}
