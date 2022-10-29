use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    };

    let s = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[i - 1][j];

            if j >= t[i - 1] {
                dp[i][j] |= dp[i - 1][j - t[i - 1]];
            }
        }
    }

    let mut answer = 0;
    for j in s / 2..=s {
        if dp[n][j] {
            answer = cmp::max(j, s - j);
            break;
        }
    }

    println!("{}", answer);
}
