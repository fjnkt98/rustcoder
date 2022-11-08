use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        wv: [(i64, i64); n]
    };

    let mut dp = vec![vec![0; x as usize + 1]; n + 1];
    for (i, &(w, v)) in wv.iter().enumerate() {
        for j in 0..=x {
            dp[i + 1][j as usize] = dp[i][j as usize];

            if j >= w {
                dp[i + 1][j as usize] =
                    std::cmp::max(dp[i + 1][j as usize], dp[i][(j - w) as usize] + v);
            }
        }
    }

    println!("{}", dp[n][x as usize]);
}
