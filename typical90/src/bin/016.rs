use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    };

    let mut answer: i64 = 1 << 60;
    for i in 0..=9999 {
        for j in 0..=9999 - i {
            let r = n - a * i - b * j;

            let k = r / c;
            if r >= 0 && a * i + b * j + c * k == n {
                answer = std::cmp::min(answer, i + j + k);
            }
        }
    }

    println!("{}", answer);
}
