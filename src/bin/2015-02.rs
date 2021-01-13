const INPUT: &str = include_str!("../data/2015-02.txt");

fn part1() {
    let mut total_area = 0;
    for line in INPUT.lines() {
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

    println!("area: {}", total_area);
}

fn part2() {
    let mut total_length = 0;
    for line in INPUT.lines() {
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

    println!("length: {}", total_length);
}

fn main() {
    part1();
    part2();
}
