use proconio::input;

fn main() {
    input! {
        mut n: i64,
    };

    let mut answer = 0.0;
    let mut m = n;
    for _ in 1..=n {
        answer += (n as f64) / (m as f64);
        m -= 1;
    }

    println!("{}", answer);
}
