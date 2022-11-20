use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut a = vec![0; t + 2];
    for &(l, r) in lr.iter() {
        a[l] += 1;
        a[r] -= 1;
    }

    let c = a.iter().cumsum().collect::<Vec<i64>>();
    println!(
        "{}",
        c[0..t]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
