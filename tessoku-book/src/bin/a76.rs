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
        w: usize,
        l: i64,
        r: i64,
        mut x: [i64; n]
    };

    let p = 1000000007;

    x.insert(0, 0);
    x.push(w as i64);

    let mut dp: Vec<i64> = vec![0; n + 2];
    dp[0] = 1;
    let mut cumsum = dp.iter().cumsum().collect::<Vec<i64>>();

    for i in 1..=n + 1 {
        let left = x.lower_bound(x[i] - r);
        let right = x.upper_bound(x[i] - l);

        if right == 0 {
            dp[i] = 0;
        } else {
            dp[i] += cumsum[right - 1];
        }

        if left > 0 {
            dp[i] -= cumsum[left - 1];
        }

        dp[i] = (dp[i] + p) % p;

        cumsum[i] = cumsum[i - 1] + dp[i];
        cumsum[i] %= p;
    }

    println!("{}", dp.iter().last().unwrap());
}
