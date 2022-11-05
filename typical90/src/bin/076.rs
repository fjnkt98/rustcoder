use itertools_num::ItertoolsNum;
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
        a: [i64; n]
    };

    let mut c = a.clone();
    c.extend(a.iter());

    let sum: i64 = a.iter().sum();
    let c = c.iter().cumsum().collect::<Vec<i64>>();
    for i in 0..n {
        let v = c[i] + sum / 10;
        let j = c.lower_bound(&v);

        let piece = c[j] - c[i];
        if piece * 10 == sum {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
