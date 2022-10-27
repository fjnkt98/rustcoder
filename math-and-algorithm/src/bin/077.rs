use itertools_num::ItertoolsNum;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    };

    let (mut x, mut y): (Vec<i64>, Vec<i64>) = xy.into_iter().unzip();
    x.sort_by_key(|x| cmp::Reverse(*x));
    y.sort_by_key(|x| cmp::Reverse(*x));

    let cx: Vec<i64> = x.iter().cumsum().collect();
    let cy: Vec<i64> = y.iter().cumsum().collect();

    let sx: i64 = x.iter().sum();
    let sy: i64 = y.iter().sum();

    let mut answer = 0;
    for i in 0..n {
        answer += (n - i - 1) as i64 * x[i] - (sx - cx[i]);
        answer += (n - i - 1) as i64 * y[i] - (sy - cy[i]);
    }

    println!("{}", answer);
}
