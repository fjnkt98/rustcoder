use proconio::input;

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

fn main() {
    input! {
        n: i32,
    };

    let mut answers: Vec<i32> = Vec::new();
    for i in 2..=n {
        if is_prime(i) {
            answers.push(i);
        }
    }

    let mut answer = String::new();
    for a in answers.iter() {
        answer.push_str(" ");
        answer.push_str(&(a.to_string()));
    }
    println!("{}", answer.trim())
}
