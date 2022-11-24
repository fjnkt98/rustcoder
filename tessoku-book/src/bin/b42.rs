use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    };

    let mut s = vec![vec![0; 2]; 4];
    for (i, p) in [-1, 1].iter().cartesian_product(&[-1, 1]).enumerate() {
        for &(a, b) in ab.iter() {
            if p.0 * a + p.1 * b > 0 {
                s[i][0] += a;
                s[i][1] += b;
            }
        }
    }
    println!(
        "{}",
        s.iter().map(|x| x[0].abs() + x[1].abs()).max().unwrap()
    );
}
