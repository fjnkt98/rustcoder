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
    /// a[k] >= x を満たす最小のkを二分探索で取得する
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

    /// a[k] > x を満たす最小のkを二分探索で取得する
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
        mut a: [i64; n],
        q: usize,
        x: [i64; q]
    };

    a.sort();

    for &x in x.iter() {
        println!("{}", a.lower_bound(x));
    }
}
