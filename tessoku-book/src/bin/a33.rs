use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut sum: i64 = 0;
    for &a in a.iter() {
        sum ^= a;
    }

    if sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
