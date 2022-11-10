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
        a: u64,
        b: u64
    };

    let p: u64 = 1000000007;
    println!("{}", a.modpow(b, p));
}
