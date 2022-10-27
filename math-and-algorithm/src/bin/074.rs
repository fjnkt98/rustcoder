use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let s: i64 = a.iter().sum();
    let c: Vec<i64> = a.iter().cumsum().collect();

    let mut answer: i64 = 0;
    for i in 0..n {
        answer += (s - c[i]) - a[i] * (n - i - 1) as i64;
    }

    println!("{}", answer);
}
