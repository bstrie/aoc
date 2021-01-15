fn part1() {
    let key = String::from("ckczppom");
    let mut num = 1;
    loop {
        let data = key.clone() + &num.to_string();
        let digest = md5::compute(data);
        let hex = format!("{:x}", digest);
        if &hex[..5] == "00000" {
            break;
        } else {
            num += 1;
        }
    }
    println!("number: {}", num);
}

fn part2() {
    let key = String::from("ckczppom");
    let mut num = 1;
    loop {
        let data = key.clone() + &num.to_string();
        let digest = md5::compute(data);
        let hex = format!("{:x}", digest);
        if &hex[..6] == "000000" {
            break;
        } else {
            num += 1;
        }
    }
    println!("number: {}", num);
}

fn main() {
    part1();
    part2();
}
