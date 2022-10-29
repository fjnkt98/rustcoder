use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [i64; n - 1]
    };

    let mut a = vec![0; n];
    a[0] = b[0];
    for i in 1..n - 1 {
        a[i] = cmp::min(b[i], b[i - 1]);
    }
    a[n - 1] = b[n - 2];

    println!("{}", a.iter().sum::<i64>());
}
