use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
        tx: [(u8, i64); q]
    };

    let mut set = BTreeSet::new();
    for &(t, x) in tx.iter() {
        if t == 1 {
            set.insert(x);
        } else {
            if set.is_empty() {
                println!("-1");
            } else {
                let right = set.range(x..).next().unwrap_or(&(1 << 60));
                let left = set.range(..x).last().unwrap_or(&(1 << 60));

                println!("{}", std::cmp::min((x - right).abs(), (x - left).abs()));
            }
        }
    }
}
