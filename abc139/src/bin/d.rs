use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    println!("{}", n * (n - 1) / 2);
}
