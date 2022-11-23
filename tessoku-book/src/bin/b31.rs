use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut answer = 0;
    answer += n / 3;
    answer += n / 5;
    answer += n / 7;
    answer += n / (3 * 5 * 7);
    answer -= n / (3 * 5);
    answer -= n / (5 * 7);
    answer -= n / (7 * 3);

    println!("{}", answer);
}
