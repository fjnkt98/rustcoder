use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        d: usize,
        xy: [(usize, i64); n]
    };

    let mut job: Vec<Vec<i64>> = vec![Vec::new(); d + 1];
    for &(x, y) in xy.iter() {
        job[x].push(y);
    }

    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut answer = 0;
    for i in 1..=d {
        for j in job[i].iter() {
            heap.push(*j);
        }

        if let Some(j) = heap.pop() {
            answer += j;
        }
    }

    println!("{}", answer);
}
