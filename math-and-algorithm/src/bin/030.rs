use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: i64,
        w: i64,
        wv: [(i64, i64); n]
    };

    let mut dp: Vec<Vec<i64>> = vec![vec![0; (w + 1) as usize]; (n + 1) as usize];
    for i in 1..=n as usize {
        for j in 0..=w as usize {
            dp[i][j] = dp[i - 1][j];

            let (w, v) = wv[i - 1];

            if (j as i64) - w >= 0 {
                dp[i][j] = cmp::max(dp[i][j], dp[i - 1][j - w as usize] + v);
            }
        }
    }

    println!("{}", dp[n as usize][w as usize]);
}
