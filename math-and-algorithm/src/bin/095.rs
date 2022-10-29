use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(i8, i64); n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let mut c1: Vec<i64> = vec![0; n];
    let mut c2: Vec<i64> = vec![0; n];
    for (i, &(c, p)) in cp.iter().enumerate() {
        if c == 1 {
            c1[i] = p;
        } else {
            c2[i] = p;
        }
    }

    c1 = c1.iter().cumsum().collect::<Vec<i64>>();
    c2 = c2.iter().cumsum().collect::<Vec<i64>>();

    c1.insert(0, 0);
    c2.insert(0, 0);

    for &(l, r) in lr.iter() {
        let a = c1[r] - c1[l - 1];
        let b = c2[r] - c2[l - 1];

        println!("{} {}", a, b);
    }
}
