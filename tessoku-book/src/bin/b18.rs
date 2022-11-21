use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    };

    let mut dp: Vec<Vec<i64>> = vec![vec![1 << 60; s + 1]; n + 1];
    let mut prev: Vec<Vec<usize>> = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[i - 1][j];
            prev[i][j] = j;

            if j >= a[i - 1] && dp[i][j] > dp[i - 1][j - a[i - 1]] + 1 {
                dp[i][j] = dp[i - 1][j - a[i - 1]] + 1;
                prev[i][j] = j - a[i - 1];
            }
        }
    }

    if dp[n][s] != 1 << 60 {
        let mut answer: Vec<usize> = Vec::new();
        let mut n = n;
        let mut s = s;
        while n != 0 || s != 0 {
            if s >= a[n - 1] && prev[n][s] == s - a[n - 1] {
                answer.push(n);
            }

            s = prev[n][s];
            n -= 1;
        }

        answer.reverse();

        println!("{}", answer.len());
        println!(
            "{}",
            answer
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("-1");
    }
}
