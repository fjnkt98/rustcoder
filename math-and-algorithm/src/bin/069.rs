use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };

    let mut answer = -(1 << 60);
    for x in [a, b].iter() {
        for y in [c, d].iter() {
            answer = cmp::max(answer, *x * *y);
        }
    }

    println!("{}", answer);
}
