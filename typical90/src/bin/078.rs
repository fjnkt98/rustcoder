use proconio::input;
use std::cmp;

pub trait Bisect<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T> Bisect<T> for [T]
where
    T: Ord,
{
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
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(a, b) in ab.iter() {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut answer = 0;
    for (node, edges) in graph.iter_mut().enumerate() {
        edges.sort();
        if edges.lower_bound(&node) == 1 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
