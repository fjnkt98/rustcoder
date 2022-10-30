use proconio::input;

fn f(n: i64) -> i64 {
    if n & 1 == 0 {
        if (n >> 1) & 1 == 0 {
            return n ^ 0;
        } else {
            return n ^ 1;
        }
    } else {
        if ((n + 1) >> 1) & 1 == 0 {
            return 0;
        } else {
            return 1;
        }
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    println!("{}", f(b) ^ f(a - 1));
}
