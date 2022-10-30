use num;
use proconio::input;
use std::ops;

pub trait ModPower<T> {
    fn modpow(&self, exp: T, modulo: T) -> T;
}

impl<T> ModPower<T> for T
where
    T: num::traits::Unsigned,
    T: ops::MulAssign,
    T: ops::RemAssign,
    T: ops::ShrAssign,
    T: ops::BitAnd<Output = T>,
    T: Copy,
{
    fn modpow(&self, mut exp: T, modulo: T) -> T {
        if exp == T::zero() {
            return T::one();
        }
        if exp == T::one() {
            return *self % modulo;
        }

        let mut x: T = *self;
        let mut r: T = T::one();

        while exp != T::zero() {
            if (exp & T::one()) == T::one() {
                r *= x;
                r %= modulo;
            }

            x *= x;
            x %= modulo;
            exp >>= T::one();
        }

        return r % modulo;
    }
}

fn comb(n: u64, r: u64, p: u64) -> u64 {
    let mut result = 1;
    for i in 0..r {
        result *= n - i;
        result %= p;
    }

    for i in 1..=r {
        result *= i.modpow(p - 2, p);
        result %= p;
    }

    return result;
}

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    };

    let p = 1000000007;

    let mut answer = 2.modpow(n, p) - 1;
    answer = answer + p - comb(n, a, p);
    answer %= p;
    answer = answer + p - comb(n, b, p);
    answer %= p;

    println!("{}", answer);
}
