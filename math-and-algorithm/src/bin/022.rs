use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    };

    let mut count = HashMap::<i64, i64>::new();
    for a in a.iter() {
        let e = count.entry(*a).or_insert(0);
        *e += 1;
    }

    let mut answer = 0;

    a.sort();
    a.dedup();
    for a in a.into_iter() {
        if a > 50000 {
            break;
        }

        if a == 50000 {
            if let Some(c) = count.get(&a) {
                answer += c * (c - 1) / 2;
            }
        } else {
            if let Some(ci) = count.get(&a) {
                if let Some(cj) = count.get(&(100000 - a)) {
                    answer += ci * cj;
                }
            }
        }
    }

    println!("{}", answer);
}
