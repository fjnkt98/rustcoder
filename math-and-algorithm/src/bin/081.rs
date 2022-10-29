use proconio::input;

fn main() {
    input! {
        mut n: i64,
    };

    let mut answer = 0;

    answer += n / 10000;
    n %= 10000;
    answer += n / 5000;
    n %= 5000;

    answer += n / 1000;

    println!("{}", answer);
}
