use itertools_num::ItertoolsNum;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    };

    a.sort_by_key(|x| cmp::Reverse(*x));

    let c: Vec<i64> = a.iter().cumsum().collect();
    let s: i64 = a.iter().sum();

    let mut answer = 0;
    for i in 0..n - 1 {
        answer += a[i] * (n - i - 1) as i64 - (s - c[i]);
    }

    println!("{}", answer);
}
