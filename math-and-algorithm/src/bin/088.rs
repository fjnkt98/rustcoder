use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    let p: i64 = 998244353;
    let sa = (a * (a + 1) / 2) % p;
    let sb = (b * (b + 1) / 2) % p;
    let sc = (c * (c + 1) / 2) % p;

    let mut answer = sa;
    answer = answer * sb % p;
    answer = answer * sc % p;
    println!("{}", answer);
}
