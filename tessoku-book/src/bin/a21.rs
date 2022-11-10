use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, i64); n]
    };

    let (p, a): (Vec<usize>, Vec<i64>) = pa.into_iter().unzip();

    let p: Vec<usize> = p.iter().map(|x| x - 1).collect();

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for l in (1..n).rev() {
        for i in 0..=n - l {
            let j = i + l;

            if i > 0 {
                dp[i][j] = std::cmp::max(
                    dp[i][j],
                    dp[i - 1][j]
                        + if i <= p[i - 1] && p[i - 1] < j {
                            a[i - 1]
                        } else {
                            0
                        },
                );
            }

            if j < n {
                dp[i][j] = std::cmp::max(
                    dp[i][j],
                    dp[i][j + 1] + if i <= p[j] && p[j] < j { a[j] } else { 0 },
                );
            }
        }
    }

    let mut answer = 0;
    for i in 0..n {
        answer = std::cmp::max(answer, dp[i][i + 1]);
    }

    println!("{}", answer);
}
