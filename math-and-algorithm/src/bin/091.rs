use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64
    };

    let mut answer = 0;
    for cmb in (1..=n).combinations(3) {
        let a = cmb[0];
        let b = cmb[1];
        let c = cmb[2];

        if a + b + c == x {
            answer += 1;
        }
    }

    println!("{}", answer);
}
