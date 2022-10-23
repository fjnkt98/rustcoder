use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let p: i64 = 1000000007;
    let mut dp: Vec<i64> = vec![0; n + 1];
    dp[1] = 1;
    dp[2] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        dp[i] %= p;
    }

    println!("{}", dp[n]);
}
