use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] * i;
    }

    println!("{}", dp[n]);
}
