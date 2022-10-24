use proconio::input;

fn main() {
    input! {
        mut n: i64,
    };

    if n % 4 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
