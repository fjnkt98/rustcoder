use proconio::input;
use std::collections::HashMap;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut b = a.clone();
    b.sort();
    b.dedup();

    let mut count: HashMap<usize, i64> = HashMap::new();
    let m = b.len();
    for a in a.iter() {
        let c = m - b.upper_bound(a);
        *count.entry(c).or_insert(0) += 1;
    }

    for i in 0..n {
        println!("{}", count.get(&i).unwrap_or(&0));
    }
}
