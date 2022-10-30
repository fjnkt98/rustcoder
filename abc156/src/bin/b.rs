use proconio::input;

fn main() {
    input! {
        mut n: i64,
        k: i64,
    };

    let mut answer = 0;
    while n != 0 {
        n /= k;
        answer += 1;
    }

    println!("{}", answer);
}
