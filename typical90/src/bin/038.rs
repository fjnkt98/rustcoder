use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64
    };

    let gcd = num_integer::gcd(a, b);

    let c = a / gcd;

    if let Some(result) = b.checked_mul(c) {
        if result > 1000000000000000000 {
            println!("Large");
        } else {
            println!("{}", result);
        }
    } else {
        println!("Large");
    }
}
