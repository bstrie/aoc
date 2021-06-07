fn main() {
    let input = "1113122113";
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut result = input.to_string();
    for _ in 0..40 {
        result = look_and_say(&result);
    }
    result.len()
}

fn part2(input: &str) -> usize {
    let mut result = input.to_string();
    for _ in 0..50 {
        result = look_and_say(&result);
    }
    result.len()
}

fn look_and_say(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut result = String::new();
    let mut run = 1;
    while let Some(c1) = chars.next() {
        match chars.peek() {
            Some(&c2) if c1 == c2 => run += 1,
            _ => {
                result.push_str(&run.to_string());
                result.push(c1);
                run = 1;
            }
        }
    }
    result
}

#[test]
fn test1() {
    aoc::test(look_and_say, [
        ("1", "11"),
        ("11", "21"),
        ("21", "1211"),
        ("1211", "111221"),
        ("111221", "312211")
    ])
}
