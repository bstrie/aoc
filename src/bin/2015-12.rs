use serde_json::Value;

fn main() {
    let input = include_str!("../data/2015-12.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    sum1(0, &v)
}

fn part2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    sum2(0, &v)
}

fn sum1(s: i64, v: &Value) -> i64 {
    match v {
        Value::Number(n) => s + n.as_i64().unwrap(),
        Value::Array(a) => a.iter().fold(s, sum1),
        Value::Object(o) => o.values().fold(s, sum1),
        _ => s
    }
}

fn sum2(s: i64, v: &Value) -> i64 {
    match v {
        Value::Number(n) => s + n.as_i64().unwrap(),
        Value::Array(a) => a.iter().fold(s, sum2),
        Value::Object(o) if o.values().all(|val| val != "red") =>
            o.values().fold(s, sum2),
        _ => s
    }
}

#[test]
fn test1() {
    aoc::test(part1, [
        ("[1,2,3]", 6),
        (r#"{"a":2,"b":4}"#, 6),
        ("[[[3]]]", 3),
        (r#"{"a":{"b":4},"c":-1}"#, 3),
        (r#"{"a":[-1,1]}"#, 0),
        (r#"[-1,{"a":1}]"#, 0),
        ("[]", 0),
        ("{}", 0)
    ]);
}

#[test]
fn test2() {
    aoc::test(part2, [
        ("[1,2,3]", 6),
        (r#"[1,{"c":"red","b":2},3]"#, 4),
        (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
        (r#"[1,"red",5]"#, 6)
    ]);
}
