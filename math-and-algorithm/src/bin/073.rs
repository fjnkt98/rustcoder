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
        mut a: [u64; n]
    };

    let modulo: u64 = 1000000007;

    let mut answer: u64 = 0;
    for (i, a) in a.iter().enumerate() {
        answer += *a * 2u64.modpow(i as u64, modulo);
        answer %= modulo;
    }

    println!("{}", answer);
}
