use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize, usize); n]
    };

    let mut nim = 0;
    for &(a, b) in ab.iter() {
        nim ^= a - 1;
        nim ^= b - 1;
    }

    if nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
