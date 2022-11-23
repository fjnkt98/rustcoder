use itertools::iproduct;
use proconio::input;

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    return ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let mut dp: Vec<Vec<f64>> = vec![vec![1e10; n]; 1 << n];
    dp[0][0] = 0.0;

    for s in 0..(1 << n) {
        for (v, u) in iproduct!(0..n, 0..n) {
            if (s >> u) & 1 == 0 && s != 0 {
                continue;
            }

            if (s >> v) & 1 == 0 {
                let d = distance(xy[u], xy[v]);
                if dp[s | (1 << v)][v] > dp[s][u] + d {
                    dp[s | (1 << v)][v] = dp[s][u] + d;
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][0]);
}
