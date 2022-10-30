use proconio::input;

fn main() {
    input! {
        H: i64,
        W: i64,
        h: i64,
        w: i64,
    };

    let mut answer = H * W;
    answer -= h * W;
    answer -= H * w;
    answer += h * w;

    println!("{}", answer);
}
