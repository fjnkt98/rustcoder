use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64
    };

    if k >= 2 * (n - 1) && (k - 2 * (n - 1)) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
