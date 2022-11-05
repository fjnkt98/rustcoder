use num;
use proconio::input;
use std::ops;

fn factorize<T>(n: T) -> Vec<(T, T)>
where
    T: num::traits::Unsigned,
    T: ops::DivAssign,
    T: ops::AddAssign,
    T: num::Zero,
    T: num::One,
    T: PartialOrd,
    T: Copy,
{
    let mut result: Vec<(T, T)> = Vec::new();
    let mut n = n;
    let mut i = T::one() + T::one();
    while i * i <= n {
        if n % i == T::zero() {
            let mut exp = T::zero();

            while n % i == T::zero() {
                exp += T::one();
                n /= i;
            }

            result.push((i, exp));
        }

        i += T::one();
    }

    if n != T::one() {
        result.push((n, T::one()));
    }

    return result;
}

fn main() {
    input! {
        n: u64,
    };

    let primes = factorize(n);
    let mut count = 0;
    for (_, exp) in primes {
        count += exp;
    }

    let mut answer = 0;
    for i in 0..20 {
        if 1 << i >= count {
            answer = i;
            break;
        }
    }

    println!("{}", answer);
}
