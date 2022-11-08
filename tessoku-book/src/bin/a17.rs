use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
        b: [i64; n - 2],
    };

    let mut dp: Vec<i64> = vec![1 << 60; n];
    let mut prev: Vec<usize> = vec![0; n];
    dp[0] = 0;
    dp[1] = a[0];
    prev[1] = 0;
    for i in 2..n {
        let step1 = dp[i - 1] + a[i - 1];
        let step2 = dp[i - 2] + b[i - 2];
        if step1 < step2 {
            dp[i] = step1;
            prev[i] = i - 1;
        } else {
            dp[i] = step2;
            prev[i] = i - 2;
        }
    }

    let mut i = n - 1;
    let mut answer = Vec::new();
    while i != 0 {
        answer.push(i);
        i = prev[i];
    }
    answer.push(0);

    println!("{}", answer.len());
    println!(
        "{}",
        answer
            .iter()
            .rev()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
