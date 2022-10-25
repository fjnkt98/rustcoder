use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    if n % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
