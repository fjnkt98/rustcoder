use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    println!("{}", n / 2 + n % 2);
}
