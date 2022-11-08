use proconio::input;
use std::cmp;
use std::collections::HashSet;

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
        let mut left: usize = 0;
        let mut right: usize = self.len();

        while right - left > 1 {
            let mid = (right + left) / 2;

            match self[mid].cmp(&x) {
                cmp::Ordering::Less | cmp::Ordering::Equal => {
                    left = mid;
                }
                cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }

        return left;
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
        let mut left: usize = 0;
        let mut right: usize = self.len();

        while right - left > 1 {
            let mid = (right + left) / 2;

            match self[mid].cmp(&x) {
                cmp::Ordering::Less => {
                    left = mid;
                }
                cmp::Ordering::Greater | cmp::Ordering::Equal => {
                    right = mid;
                }
            }
        }

        return right;
    }
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    };

    let mut ab = Vec::with_capacity(n * n);
    for &a in a.iter() {
        for &b in b.iter() {
            ab.push(a + b);
        }
    }
    ab.sort();

    let mut cd: HashSet<i64> = HashSet::new();
    for &c in c.iter() {
        for &d in d.iter() {
            cd.insert(c + d);
        }
    }

    for &ab in ab.iter() {
        if cd.contains(&(k - ab)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
