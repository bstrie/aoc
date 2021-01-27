fn main() {
    let input = "ckczppom";
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut n = 1;
    loop {
        let data = format!("{}{}", input, n);
        let hash = format!("{:x}", md5::compute(data));
        if &hash[..5] == "00000" {
            return n;
        }
        n += 1;
    }
}

fn part2(input: &str) -> i32 {
    let mut n = 1;
    loop {
        let data = format!("{}{}", input, n);
        let hash = format!("{:x}", md5::compute(data));
        if &hash[..6] == "000000" {
            return n;
        }
        n += 1;
    }
}

#[test]
#[ignore]
fn test1() {
    aoc::test(part1, [
        ("abcdef", 609043),
        ("pqrstuv", 1048970)
    ])
}
