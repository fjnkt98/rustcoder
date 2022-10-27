use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(i64, i64, i64); n]
    };

    let mut answer: f64 = 0.0;
    for cmb in (0..n).combinations(2) {
        let (ai, bi, ci) = abc[cmb[0]];
        let (aj, bj, cj) = abc[cmb[1]];

        if ai * bj - aj * bi == 0 {
            continue;
        }

        let x2 = (ai * cj - aj * ci) as f64 / (ai * bj - aj * bi) as f64;
        let x1 = (ci as f64 - bi as f64 * x2) as f64 / ai as f64;

        if x1 < 0.0 || x2 < 0.0 {
            continue;
        }

        let mut ok = true;
        for (a, b, c) in abc.iter() {
            if *a as f64 * (x1 - 1e-6) + *b as f64 * (x2 - 1e-6) > *c as f64 {
                ok = false;
            }
        }

        if ok {
            if answer < x1 + x2 {
                answer = x1 + x2;
            }
        }
    }
    println!("{}", answer);
}
