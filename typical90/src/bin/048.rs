use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(i64, i64); n]
    };

    let mut c = Vec::with_capacity(2 * n);
    for &(a, b) in ab.iter() {
        c.push(b);
        c.push(a - b);
    }

    c.sort_by_key(|&x| cmp::Reverse(x));

    println!("{}", &c[0..k].iter().sum::<i64>());
}
