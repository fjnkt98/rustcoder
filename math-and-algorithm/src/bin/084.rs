use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    if c - a - b > 0 && 4 * a * b < (c - a - b).pow(2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
