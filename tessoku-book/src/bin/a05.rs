use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    };

    let mut answer = 0;
    for (a, b) in iproduct!(1..=n, 1..=n) {
        let c = k - a - b;

        if 1 <= c && c <= n && a + b + c == k {
            answer += 1;
        }
    }

    println!("{}", answer);
}
