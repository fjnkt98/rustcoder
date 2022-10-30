use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    let m = x / 100;
    if 100 * m <= x && x <= 105 * m {
        println!("1");
    } else {
        println!("0");
    }
}
