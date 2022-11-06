use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut c = vec![0; d + 2];
    for &(l, r) in lr.iter() {
        c[l] += 1;
        c[r + 1] -= 1;
    }

    c = c.iter().cumsum().collect::<Vec<i64>>();
    for c in c[1..d + 1].iter() {
        println!("{}", c);
    }
}
