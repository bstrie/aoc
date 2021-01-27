#![feature(array_windows)]

fn main() {
    let input = "hxbxwxba";
    let p1 = part1(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", part1(&p1));
}

fn part1(input: &str) -> String {
    let mut pw = input.as_bytes().to_vec();
    loop {
        advance(&mut pw);
        if has_straight(&pw) && has_pairs(&pw) && no_forbidden(&pw) {
            break;
        }
    }
    std::str::from_utf8(&pw).unwrap().to_string()
}

fn advance(buf: &mut Vec<u8>) {
    for c in buf.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
}

fn has_straight(buf: &[u8]) -> bool {
    for &[c1, c2, c3] in buf.array_windows() {
        if c1+1 == c2 && c2+1 == c3 {
            return true;
        }
    }
    false
}

fn has_pairs(buf: &[u8]) -> bool {
    let mut one_pair = false;
    let mut win = buf.array_windows();
    while let Some(&[c1, c2]) = win.next() {
        if c1 == c2 {
            if !one_pair {
                one_pair = true;
                win.next();
            } else {
                return true;
            }
        }
    }
    false
}

fn no_forbidden(buf: &[u8]) -> bool {
    buf.iter().all(|&c| c != b'i' && c != b'l' && c != b'o')
}

#[test]
fn test1() {
    aoc::test(part1, [
        ("abcdefgh", "abcdffaa"),
        ("ghijklmn", "ghjaabcc")
    ]);
}
