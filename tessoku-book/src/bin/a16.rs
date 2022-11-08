use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
        b: [i64; n - 2],
    };

    let mut dp: Vec<i64> = vec![1 << 60; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = cmp::min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    println!("{}", dp[n - 1]);
}
