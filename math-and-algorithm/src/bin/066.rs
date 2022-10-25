use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: i64,
        k: i64,
    };

    let mut count = 0;
    for a in 1..=n {
        for b in cmp::max(1, a - (k - 1))..=cmp::min(n, a + (k - 1)) {
            for c in cmp::max(1, b - (k - 1))..=cmp::min(n, b + (k - 1)) {
                if (c - a).abs() <= k - 1 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", n.pow(3) - count);
}
