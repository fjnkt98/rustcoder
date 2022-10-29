use proconio::input;

fn divisors(n: i64) -> Vec<i64> {
    let mut result = Vec::new();

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            result.push(i);

            if n / i != i {
                result.push(n / i);
            }
        }

        i += 1;
    }

    return result;
}

fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    };

    for ab in divisors(y).iter() {
        for a in divisors(*ab).iter() {
            if *a > n {
                continue;
            }
            let b = ab / a;
            if b > n {
                continue;
            }

            let cd = y / ab;
            for c in divisors(cd).iter() {
                if *c > n {
                    continue;
                }
                let d = cd / c;
                if d > n {
                    continue;
                }

                if a + b + c + d == x {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
