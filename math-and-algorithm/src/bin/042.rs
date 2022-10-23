use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let mut answer: i64 = 0;
    for i in 1..=n {
        let k = n / i as i64;

        answer += k * (k + 1) * i as i64 / 2;
    }

    println!("{}", answer);
}
