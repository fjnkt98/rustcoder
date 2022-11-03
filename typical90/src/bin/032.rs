use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        m: usize,
        xy: [(usize, usize); m]
    };

    let mut r: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &(x, y) in xy.iter() {
        r.entry(x - 1).or_insert(HashSet::new()).insert(y - 1);
        r.entry(y - 1).or_insert(HashSet::new()).insert(x - 1);
    }

    let mut answer: i64 = 1 << 60;
    for p in (0..n).permutations(n) {
        let mut ok = true;
        for (prev, next) in p.iter().tuple_windows() {
            if let Some(a) = r.get(prev) {
                if a.contains(next) {
                    ok = false;
                }
            }
        }

        let mut sum: i64 = 0;
        for (i, &p) in p.iter().enumerate() {
            sum += a[p][i];
        }

        if ok {
            answer = std::cmp::min(answer, sum);
        }
    }

    if answer == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", answer);
    }
}
