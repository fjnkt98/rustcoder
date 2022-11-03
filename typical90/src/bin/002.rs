use proconio::input;

fn check(s: &str) -> bool {
    let mut depth = 0;
    for c in s.chars() {
        if c == '(' {
            depth += 1;
        } else {
            depth -= 1;
        }

        if depth < 0 {
            return false;
        }
    }

    if depth == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    input! {
        n: i64,
    };

    if n % 2 == 1 {
        return;
    }

    let mut answer: Vec<String> = Vec::new();
    for bits in 0..(1 << n) {
        let mut s = String::new();
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                s.push('(');
            } else {
                s.push(')');
            }
        }

        if check(&s) {
            answer.push(s);
        }
    }

    answer.sort();
    println!("{}", answer.join("\n"));
}
