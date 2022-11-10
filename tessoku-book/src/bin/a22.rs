use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 1],
    };

    let a = a.iter().map(|x| x - 1).collect::<Vec<usize>>();
    let b = b.iter().map(|x| x - 1).collect::<Vec<usize>>();

    let mut dp: Vec<i64> = vec![-(1 << 60); n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[a[i]] = std::cmp::max(dp[a[i]], dp[i] + 100);
        dp[b[i]] = std::cmp::max(dp[b[i]], dp[i] + 150);
    }

    println!("{}", dp[n - 1]);
}
