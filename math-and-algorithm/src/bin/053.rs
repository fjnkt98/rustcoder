use proconio::input;

fn iterative_power(a: i64, n: i64, p: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    if n == 1 {
        return a % p;
    }

    if n % 2 == 1 {
        return (a * iterative_power(a, n - 1, p)) % p;
    }

    let t = iterative_power(a, n / 2, p);

    return (t * t) % p;
}

fn main() {
    input! {
        n: i64,
    };

    let p = 1000000007;
    let mut answer = iterative_power(4, n + 1, p) - 1;
    answer = answer * iterative_power(3, p - 2, p);
    answer %= p;

    println!("{}", answer);
}
