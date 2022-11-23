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
        mut xy: [(i64, i64); n]
    };

    // Xについて昇順ソート、Yについて降順ソート
    // Xの値が等しい箱同士は重ねられない。Yについても昇順ソートしてしまうと、Xの値が同じである箱のうちのいくつかが最小増加部分列に採用されてしまう。
    // そこでYについては降順ソートにしておくことで、Xの値が同じである箱のYは降順になるので、たかだか1個しか最小増加部分列に採用されなくなる
    xy.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1).reverse()));
    let (_, y): (Vec<i64>, Vec<i64>) = xy.iter().cloned().unzip();

    let mut dp: Vec<i64> = vec![1 << 60; n + 1];
    for i in 0..n {
        let j = dp.lower_bound(y[i]);
        dp[j] = y[i];
    }

    println!("{:?}", dp.lower_bound(1 << 60));
}
