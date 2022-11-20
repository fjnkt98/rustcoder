use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mid = n / 2;
    let mut left = HashSet::new();
    for bits in 0..1i64 << mid {
        let mut sum = 0;
        for i in 0..mid {
            if (bits >> i) & 1 == 1 {
                sum += a[i];
            }
        }

        left.insert(sum);
    }

    let mut right = HashSet::new();
    for bits in 0..1i64 << (n - mid) {
        let mut sum = 0;
        for i in 0..(n - mid) {
            if (bits >> i) & 1 == 1 {
                sum += a[i + mid];
            }
        }

        right.insert(sum);
    }

    for l in left.iter() {
        if right.contains(&(k - *l)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
