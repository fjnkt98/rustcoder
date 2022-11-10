use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h],
    };

    let s = s
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut dp = vec![vec![0i64; w]; h];
    dp[0][0] = 1;

    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '#' {
            continue;
        }
        if i > 0 {
            dp[i][j] += dp[i - 1][j];
        }

        if j > 0 {
            dp[i][j] += dp[i][j - 1];
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
