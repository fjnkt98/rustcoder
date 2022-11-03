use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i64; n]
    };

    println!("{}", t.iter().min().unwrap());
}
