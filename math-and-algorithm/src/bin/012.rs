use proconio::input;

fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}

fn main() {
    input! {
        n: i64,
    };

    if is_prime(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}
