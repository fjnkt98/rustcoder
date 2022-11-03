use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    };

    let gcd = gcd(gcd(a, b), c);
    let mut answer = 0;
    answer += a / gcd - 1;
    answer += b / gcd - 1;
    answer += c / gcd - 1;

    println!("{}", answer);
}
