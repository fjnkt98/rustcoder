use itertools::Itertools;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    };

    let mut answer = 1 << 60;
    for cmb in (0..n).combinations(2) {
        let (x1, y1) = xy[cmb[0]];
        let (x2, y2) = xy[cmb[1]];

        let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);

        answer = cmp::min(answer, dist);
    }

    println!("{}", (answer as f64).sqrt());
}
