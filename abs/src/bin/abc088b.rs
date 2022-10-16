use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: i32,
        mut a: [i32;n],
    };

    a.sort_by_key(|&x| Reverse(x));

    let mut alice = 0;
    let mut bob = 0;
    for (i, a) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += a;
        } else {
            bob += a;
        }
    }

    println!("{}", alice - bob);
}
