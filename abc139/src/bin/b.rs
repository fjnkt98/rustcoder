use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    let mut tap = 1;
    let mut answer = 0;
    while tap < b {
        tap += a - 1;
        answer += 1;
    }

    println!("{}", answer);
}
