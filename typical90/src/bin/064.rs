use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(usize, usize, i64); q],
    };

    let mut diff = a
        .iter()
        .tuple_windows()
        .map(|(prev, next)| next - prev)
        .collect::<Vec<i64>>();

    let mut answer = diff.iter().map(|&x| x.abs()).sum::<i64>();

    for &(l, r, v) in lrv.iter() {
        let l = l - 1;
        let r = r - 1;

        if l > 0 {
            answer -= diff[l - 1].abs();
            diff[l - 1] += v;
            answer += diff[l - 1].abs();
        }
        if r < n - 1 {
            answer -= diff[r].abs();
            diff[r] -= v;
            answer += diff[r].abs();
        }

        println!("{}", answer);
    }
}
