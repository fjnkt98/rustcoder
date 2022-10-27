use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    let mut answer = 0;
    for t in 1..=b {
        if (b / t) - ((a + t - 1) / t) >= 1 {
            answer = t;
        }
    }

    println!("{}", answer);
}
