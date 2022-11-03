use proconio::input;
use std::cmp;

pub trait Bisect<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut left: usize = 0;
        let mut right: usize = self.len();

        while left != right {
            let mid = (right + left) / 2;

            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    left = mid + 1;
                }
                cmp::Ordering::Equal | cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }

        return left;
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut left: usize = 0;
        let mut right: usize = self.len();

        while right != left {
            let mid = (right + left) / 2;

            match self[mid].cmp(x) {
                cmp::Ordering::Less | cmp::Ordering::Equal => {
                    left = mid + 1;
                }
                cmp::Ordering::Greater => {
                    right = mid;
                }
            }
        }

        return left;
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q]
    };

    a.sort();

    for &b in b.iter() {
        let i = a.lower_bound(&b);

        let mut v = if i == n {
            (b - a[i - 1]).abs()
        } else {
            (b - a[i]).abs()
        };

        if i > 0 {
            v = cmp::min(v, (b - a[i - 1]).abs());
        }
        if i + 1 < n {
            v = cmp::min(v, (b - a[i + 1]).abs());
        }

        println!("{}", v);
    }
}
