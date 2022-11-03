use proconio::input;

fn encode(s: &str) -> String {
    let mut base10: u64 = 0;
    for (i, c) in s.chars().rev().enumerate() {
        base10 += c.to_digit(10).unwrap() as u64 * 8u64.pow(i as u32);
    }

    let mut result = String::new();
    while base10 != 0 {
        result.push(std::char::from_digit((base10 % 9) as u32, 10).unwrap());
        base10 /= 9;
    }

    return result.chars().rev().collect::<String>();
}

fn main() {
    input! {
        mut n: String,
        k: u64
    };

    if n == "0" {
        println!("0");
        return;
    }

    for _ in 0..k {
        n = encode(&n);
        n = n.replace("8", "5");
    }

    println!("{}", n);
}
