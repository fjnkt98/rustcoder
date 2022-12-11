use proconio::input;
use std::collections::HashSet;

fn judge(a: &Vec<i64>, t: i64) -> bool {
    let mut s0 = HashSet::new();
    let mut s1 = HashSet::new();
    s1.insert(0);

    for i in 1..=a.len() {
        if i % 2 == 0 {
            s1.clear();
            for s in s0.iter() {
                let right = s + a[i - 1];
                if right.abs() <= 10000 {
                    s1.insert(right);
                }

                let left = s - a[i - 1];
                if left.abs() <= 10000 {
                    s1.insert(left);
                }
            }
        } else {
            s0.clear();
            for s in s1.iter() {
                let right = s + a[i - 1];
                if right.abs() <= 10000 {
                    s0.insert(right);
                }

                let left = s - a[i - 1];
                if left.abs() <= 10000 {
                    s0.insert(left);
                }
            }
        }
    }
    if a.len() % 2 == 0 {
        return s1.contains(&t);
    } else {
        return s0.contains(&t);
    }
}

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n]
    };

    let mut h = Vec::new();
    let mut v = Vec::new();
    for (i, &a) in a[1..].iter().enumerate() {
        if i % 2 == 1 {
            h.push(a);
        } else {
            v.push(a);
        }
    }

    if judge(&h, x - a[0]) && judge(&v, y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
