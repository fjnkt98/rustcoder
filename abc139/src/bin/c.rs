use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    };

    let mut answer = 0;
    let mut count = 0;
    for i in 0..n - 1 {
        if h[i] >= h[i + 1] {
            count += 1;
        } else {
            answer = cmp::max(answer, count);
            count = 0;
        }
    }
    answer = cmp::max(answer, count);

    println!("{}", answer);
}
