use proconio::input;

fn f(n: usize) -> usize {
    let s = n
        .to_string()
        .chars()
        .map(|c| c as usize - 48)
        .collect::<Vec<usize>>();
    return n - s.iter().sum::<usize>();
}

fn main() {
    input! {
        n: usize,
        k: usize
    };

    let m = (k as f64).log2().ceil() as usize;

    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
    for j in 1..=n {
        dp[0][j] = f(j);
    }

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j] as usize];
        }
    }

    for i in 1..=n {
        let mut answer = i;
        let mut bit: usize = 0;
        let mut k = k;
        while k != 0 {
            if k & 1 == 1 {
                answer = dp[bit][answer] as usize;
            }

            k >>= 1;
            bit += 1;
        }

        println!("{}", answer);
    }
}
