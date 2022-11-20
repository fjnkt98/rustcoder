use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    for cmb in (0..n).combinations(3) {
        let i = cmb[0];
        let j = cmb[1];
        let k = cmb[2];

        if a[i] + a[j] + a[k] == 1000 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
