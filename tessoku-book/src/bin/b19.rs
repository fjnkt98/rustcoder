use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        wv: [(usize, usize); n]
    };

    let (w, v): (Vec<usize>, Vec<usize>) = wv.iter().cloned().unzip();

    let sum: usize = v.iter().sum();
    let mut dp: Vec<Vec<usize>> = vec![vec![1 << 60; sum + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=sum {
            dp[i][j] = dp[i - 1][j];

            if j >= v[i - 1] {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - v[i - 1]] + w[i - 1]);
            }
        }
    }

    let mut answer = 0;
    for j in (0..=sum).rev() {
        if dp[n][j] <= x {
            answer = j;
            break;
        }
    }

    println!("{}", answer);
}
