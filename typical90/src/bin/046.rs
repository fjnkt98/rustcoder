use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    };

    let mut count_a: HashMap<i64, i64> = HashMap::new();
    for &a in a.iter() {
        *count_a.entry(a % 46).or_insert(0) += 1;
    }
    let mut count_b: HashMap<i64, i64> = HashMap::new();
    for &b in b.iter() {
        *count_b.entry(b % 46).or_insert(0) += 1;
    }
    let mut count_c: HashMap<i64, i64> = HashMap::new();
    for &c in c.iter() {
        *count_c.entry(c % 46).or_insert(0) += 1;
    }

    let mut answer = 0;
    for (&ka, &va) in count_a.iter() {
        for (&kb, &vb) in count_b.iter() {
            for (&kc, &vc) in count_c.iter() {
                if (ka + kb + kc) % 46 == 0 {
                    answer += va * vb * vc;
                }
            }
        }
    }

    println!("{}", answer);
}
