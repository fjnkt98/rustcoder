use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for j in 1..=n {
        dp[n][j] = a[j - 1];
    }

    for i in (1..n).rev() {
        for j in 1..=i {
            if i % 2 == 0 {
                dp[i][j] = std::cmp::min(dp[i + 1][j], dp[i + 1][j + 1]);
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i + 1][j + 1]);
            }
        }
    }

    println!("{}", dp[1][1]);
}
