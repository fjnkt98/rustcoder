use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n]
    };

    let sum: i64 = a.iter().sum();

    if k >= sum && (k - sum) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
