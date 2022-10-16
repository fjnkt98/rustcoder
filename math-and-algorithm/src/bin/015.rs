use num::Integer;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    println!("{}", Integer::gcd(&a, &b));
}
