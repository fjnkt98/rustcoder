use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut answer = a[0];
    for a in a.iter() {
        answer = Integer::gcd(&answer, a);
    }

    println!("{}", answer);
}
