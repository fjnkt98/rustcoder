use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize); q]
    };

    let a = a.iter().map(|x| x - 1).collect::<Vec<usize>>();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; 64];
    for j in 0..n {
        dp[0][j] = a[j];
    }

    for i in 1..64 {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for &(x, y) in xy.iter() {
        let mut x = x - 1;
        let mut y = y;

        let mut i = 0;
        while y != 0 {
            if y & 1 == 1 {
                x = dp[i][x];
            }

            y >>= 1;
            i += 1;
        }
        println!("{}", x + 1);
    }
}
