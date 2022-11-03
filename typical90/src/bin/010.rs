use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(i8, i64); n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let mut c1 = vec![0; n + 1];
    let mut c2 = vec![0; n + 1];
    for (i, &(c, p)) in cp.iter().enumerate() {
        if c == 1 {
            c1[i + 1] = p;
        } else {
            c2[i + 1] = p;
        }
    }

    c1 = c1.iter().cumsum().collect::<Vec<i64>>();
    c2 = c2.iter().cumsum().collect::<Vec<i64>>();

    for &(l, r) in lr.iter() {
        let a1 = c1[r] - c1[l - 1];
        let a2 = c2[r] - c2[l - 1];

        println!("{} {}", a1, a2);
    }
}
