use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut dp = vec![0; n + 1];
    let p = 1000000007;
    dp[1] = 1;
    dp[2] = 1;
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        dp[i] %= p;
    }

    println!("{}", dp[n]);
}
