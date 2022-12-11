use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    };

    let p: i64 = 998244353;
    let a = a % p;
    let b = b % p;
    let c = c % p;
    let d = d % p;
    let e = e % p;
    let f = f % p;

    let mut left = a;
    left *= b;
    left %= p;
    left *= c;
    left %= p;

    let mut right = d;
    right *= e;
    right %= p;
    right *= f;
    right %= p;

    if left < right {
        left += p;
    }

    println!("{}", (left - right) % p);
}
