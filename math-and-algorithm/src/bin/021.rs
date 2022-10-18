use proconio::input;

fn main() {
    input! {
        n: i64,
        r: i64,
    };

    let mut answer: i64 = 1;
    for i in 1..=n {
        answer *= i;
    }
    for i in 1..=r {
        answer /= i;
    }
    for i in 1..=(n - r) {
        answer /= i;
    }

    println!("{}", answer);
}
