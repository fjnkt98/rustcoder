use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n]
    };

    td.sort_by_key(|x| x.1);

    let (t, d): (Vec<usize>, Vec<usize>) = td.iter().copied().unzip();

    let time: usize = 1500;
    let mut dp: Vec<Vec<i64>> = vec![vec![-(1 << 60); time + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=time {
            dp[i + 1][j] = std::cmp::max(dp[i + 1][j], dp[i][j]);

            if j + t[i] > d[i] {
                continue;
            }

            dp[i + 1][j + t[i]] = std::cmp::max(dp[i + 1][j + t[i]], dp[i][j] + 1);
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
