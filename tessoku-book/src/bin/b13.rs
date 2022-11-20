use itertools_num::ItertoolsNum;
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
        k: i64,
        a: [i64; n]
    };

    let mut c = a.iter().cumsum().collect::<Vec<i64>>();
    c.insert(0, 0);
    let mut answer = 0;
    for i in 0..=n {
        let index = c.upper_bound(c[i] + k);
        answer += index - i - 1;
    }

    println!("{}", answer);
}
