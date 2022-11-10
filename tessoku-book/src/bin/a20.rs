use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let n = s.len();
    let m = t.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                if dp[i - 1][j] < dp[i][j - 1] {
                    dp[i][j] = dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
    }

    println!("{}", dp[n][m]);
}
