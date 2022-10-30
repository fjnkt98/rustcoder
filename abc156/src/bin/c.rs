use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        x: [i64; n]
    };

    let mut answer: i64 = 1 << 60;
    for p in 0..=100 {
        let mut sum = 0;
        for j in 0..n {
            sum += (x[j] - p).pow(2);
        }

        answer = cmp::min(answer, sum);
    }

    println!("{}", answer);
}
