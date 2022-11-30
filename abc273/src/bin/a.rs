use proconio::input;

fn f(x: i64) -> i64 {
    if x == 0 {
        return 1;
    } else {
        return x * f(x - 1);
    }
}

fn main() {
    input! {
        n: i64,
    };

    println!("{}", f(n));
}
