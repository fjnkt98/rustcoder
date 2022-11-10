use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i8; n]; m]
    };

    let mut dp: Vec<Vec<i64>> = vec![vec![1 << 60; 1 << n]; m + 1];
    dp[0][0] = 0;

    for i in 0..m {
        for s in 0..(1 << n) {
            dp[i + 1][s] = std::cmp::min(dp[i + 1][s], dp[i][s]);

            let mut t: usize = 0;
            for (j, &a) in a[i].iter().enumerate() {
                if a == 1 {
                    t |= 1 << j;
                }
            }

            dp[i + 1][s | t] = std::cmp::min(dp[i + 1][s | t], dp[i][s] + 1);
        }
    }

    let answer = dp[m][(1 << n) - 1];
    println!("{}", if answer == 1 << 60 { -1 } else { answer });
}
