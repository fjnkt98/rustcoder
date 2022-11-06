use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q]
    };

    let c = a.iter().cumsum().collect::<Vec<i64>>();
    for &(l, r) in lr.iter() {
        let l = l - 1;
        let r = r - 1;

        println!("{}", c[r] - if l > 0 { c[l - 1] } else { 0 });
    }
}
