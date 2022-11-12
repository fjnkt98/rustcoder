use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: i64,
        a: [i64; n],
        c: [i64; m],
    };

    let answer: i64 =
        m as i64 * a.iter().sum::<i64>() + n as i64 * c.iter().sum::<i64>() + (n * m) as i64 * b;

    println!("{}", answer);
}
