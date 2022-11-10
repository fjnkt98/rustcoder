use proconio::input;
use std::cmp;

pub trait Bisect<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T> Bisect<T> for [T]
where
    T: Ord,
    T: Copy,
{
    /// a[k] <= x を満たす最小のkを二分探索で取得する
    ///
    /// ## Args
    ///
    /// * `x` - 探索対象の値
    ///
    /// ## Returns
    ///
    /// * `usize` - インデックス
    ///
    fn lower_bound(&self, x: T) -> usize {
        let mut left: i32 = -1;
        let mut right: i32 = self.len() as i32;

        while right - left > 1 {
            let mid = (right + left) / 2;

            match self[mid as usize].cmp(&x) {
                cmp::Ordering::Less => {
                    left = mid;
                }
                cmp::Ordering::Greater | cmp::Ordering::Equal => {
                    right = mid;
                }
            }
        }

        return right as usize;
    }

    /// a[k] < x を満たす最小のkを二分探索で取得する
    ///
    /// ## Args
    ///
    /// * `x` - 探索対象の値
    ///
    /// ## Returns
    ///
    /// * `usize` - インデックス
    ///
    fn upper_bound(&self, x: T) -> usize {
        let mut left: i32 = -1;
        let mut right: i32 = self.len() as i32;

        while right - left > 1 {
            let mid = (right + left) / 2;

            match self[mid as usize].cmp(&x) {
                cmp::Ordering::Less | cmp::Ordering::Equal => {
                    left = mid;
                }
                cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }

        return right as usize;
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut dp: Vec<i64> = vec![1 << 60; n + 1];

    for i in 0..n {
        let j = dp.lower_bound(a[i]);
        dp[j] = a[i];
    }

    println!("{}", dp.lower_bound(1 << 60));
}
