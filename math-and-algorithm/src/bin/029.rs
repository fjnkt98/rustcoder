use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("{}", 1);
        return;
    }
    if n == 2 {
        println!("{}", 2);
        return;
    }

    let mut dp: Vec<i64> = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[n]);
}
