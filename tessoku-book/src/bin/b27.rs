use num_integer::lcm;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    };

    println!("{}", lcm(a, b));
}
