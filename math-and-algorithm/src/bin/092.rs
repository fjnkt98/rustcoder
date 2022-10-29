use proconio::input;
use std::cmp;

fn divisors(n: i64) -> Vec<i64> {
    let mut result = Vec::new();

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            result.push(i);

            if n / i != i {
                result.push(n / i);
            }
        }

        i += 1;
    }

    return result;
}

fn main() {
    input! {
        n: i64,
    };

    let divisors = divisors(n);
    let mut answer: i64 = 1 << 60;
    for d in divisors.iter() {
        answer = cmp::min(answer, 2 * (d + n / d));
    }

    println!("{}", answer);
}
