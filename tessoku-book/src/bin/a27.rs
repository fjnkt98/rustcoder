use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    };

    println!("{}", gcd(a, b));
}
