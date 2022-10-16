use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [i32; n],
    };

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            dp[i][j] = dp[(i - 1)][j];

            if (j as i32) - a[i - 1] >= 0 && dp[(i - 1)][(j - (a[(i - 1)] as usize))] {
                dp[i][j] = true;
            }
        }
    }

    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No")
    }
}
