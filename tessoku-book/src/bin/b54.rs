use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut count: HashMap<i64, i64> = HashMap::new();
    for &a in a.iter() {
        *count.entry(a).or_insert(0) += 1;
    }

    let mut answer = 0;
    for v in count.values() {
        answer += v * (v - 1) / 2;
    }

    println!("{}", answer);
}
