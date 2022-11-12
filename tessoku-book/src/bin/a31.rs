use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    let mut answer: i64 = 0;
    answer += n / 3;
    answer += n / 5;
    answer -= n / 15;

    println!("{}", answer);
}
