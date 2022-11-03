use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
