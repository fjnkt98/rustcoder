use proconio::input;

fn main() {
    input! {
        n: i64,
        mut k: i64,
        mut a: [i64; n]
    };

    a = a.iter().map(|x| x - 1).collect();

    let mut dp = vec![vec![0i64; n as usize]; (64) as usize];
    for j in 0..(n as usize) {
        dp[0][j] = a[j];
    }

    for i in 1..64 {
        for j in 0..n {
            dp[i as usize][j as usize] =
                dp[(i - 1) as usize][(dp[(i - 1) as usize][j as usize]) as usize];
        }
    }

    let mut answer: i64 = 0;
    let mut i = 0;
    while k != 0 {
        if k & 1 == 1 {
            answer = dp[i][answer as usize];
        }

        k >>= 1;
        i += 1;
    }

    println!("{}", answer + 1);
}
