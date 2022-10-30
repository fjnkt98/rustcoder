use proconio::input;

fn main() {
    input! {
        m1: i64,
        d1: i64,
        m2: i64,
        d2: i64,
    };

    if d2 == 1 {
        println!("1");
    } else {
        println!("0");
    }
}
