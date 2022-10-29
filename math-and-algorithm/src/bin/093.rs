use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: i64,
        mut b: i64,
    };

    let gcd = gcd(a, b);

    b /= gcd;

    if a > std::i64::MAX / b {
        println!("Large");
    } else {
        if a * b > 1000000000000000000 {
            println!("Large");
        } else {
            println!("{}", a * b);
        }
    }
}
