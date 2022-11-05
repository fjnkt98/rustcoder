use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; 6]; n]
    };

    let p = 1000000007;
    let mut dp = vec![vec![0i64; 6]; n];
    dp[0] = a[0].clone();
    for i in 1..n {
        for j in 0..6 {
            for k in 0..6 {
                dp[i][j] += dp[i - 1][k] * a[i][j];
                dp[i][j] %= p;
            }
        }
    }

    println!("{}", dp[n - 1].iter().sum::<i64>() % p);
}
