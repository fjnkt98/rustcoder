use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    let n = s.len();
    let m = t.len();

    let mut dp: Vec<Vec<i64>> = vec![vec![1 << 60; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            if i > 0 && j > 0 {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 1);
                }
            }

            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }
            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
        }
    }

    println!("{}", dp[n][m]);
}
