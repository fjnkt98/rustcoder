use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64;n],
        lr: [(i64, i64);q]
    };

    let c = a.iter().cumsum().collect::<Vec<i64>>();

    for (l, r) in lr.iter() {
        let mut answer = c[(r - 1) as usize];
        if *l > 1 {
            answer -= c[(l - 2) as usize];
        }
        println!("{}", answer);
    }
}
