use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: i64,
        a: [i64; n]
    };

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 2]; (n + 1) as usize];

    for i in 1..=n as usize {
        dp[i][0] = cmp::max(dp[i - 1][0], dp[i - 1][1]);

        dp[i][1] = dp[i - 1][0] + a[i - 1];
    }

    println!("{}", cmp::max(dp[n as usize][0], dp[n as usize][1]));
}
