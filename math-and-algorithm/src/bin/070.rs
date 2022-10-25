use itertools::iproduct;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut xy: [(i64, i64); n]
    };

    let mut answer: i64 = 1i64 << 62;
    for (i, j, k, l) in iproduct!(0..n, 0..n, 0..n, 0..n) {
        let left = xy[i].0;
        let right = xy[j].0;
        let top = xy[k].1;
        let bottom = xy[l].1;

        let mut count = 0;
        for (x, y) in xy.iter() {
            if (left <= *x && *x <= right) && (bottom <= *y && *y <= top) {
                count += 1;
            }
        }

        if count >= x {
            answer = cmp::min(answer, (right - left) * (top - bottom));
        }
    }

    println!("{}", answer);
}
