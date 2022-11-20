use itertools::{iproduct, Itertools};
use proconio::input;

fn inversion(a: &Vec<i64>) -> i64 {
    let mut result = 0;
    for cmb in (0..a.len()).combinations(2) {
        let i = cmb[0];
        let j = cmb[1];

        if a[i] > a[j] {
            result += 1;
        }
    }

    return result;
}

fn main() {
    input! {
        n: usize,
        p: [[i64; n]; n]
    };

    let mut row: Vec<i64> = vec![0; n];
    let mut col: Vec<i64> = vec![0; n];

    for (i, j) in iproduct!(0..n, 0..n) {
        if p[i][j] != 0 {
            row[i] = p[i][j];
            col[j] = p[i][j];
        }
    }

    println!("{}", inversion(&row) + inversion(&col));
}
