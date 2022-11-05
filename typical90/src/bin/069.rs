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

fn main() {
    input! {
        n: u64,
        k: u64,
    };

    if k == 1 {
        if n == 1 {
            println!("1");
        } else {
            println!("0");
        }
        return;
    }

    if n == 1 {
        println!("{}", k);
        return;
    }

    let p = 1000000007;
    let mut answer = k * (k - 1) % p;
    answer *= (k - 2).modpow(n - 2, p);
    answer %= p;
    println!("{}", answer);
}
