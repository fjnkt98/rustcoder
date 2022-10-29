use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let p: i64 = 1000000007;
    let answer: i64 = (n * (n + 1) / 2) % p;
    println!("{}", answer.pow(2) % p);
}
