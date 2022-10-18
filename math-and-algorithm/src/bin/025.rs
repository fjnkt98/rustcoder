use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
        b: [i64; n],
    };

    let mut answer = 0.0;
    for (a, b) in a.iter().zip(b.iter()) {
        answer += 2.0 * (*a as f64) / 6.0;
        answer += 4.0 * (*b as f64) / 6.0;
    }

    println!("{}", answer);
}
