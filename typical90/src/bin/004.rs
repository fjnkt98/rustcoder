use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    };

    let mut row: Vec<i64> = vec![0; h];
    let mut col: Vec<i64> = vec![0; w];

    for (i, j) in iproduct!(0..h, 0..w) {
        row[i] += a[i][j];
        col[j] += a[i][j];
    }

    let mut b = vec![vec![0; w]; h];
    for (i, j) in iproduct!(0..h, 0..w) {
        b[i][j] = row[i] + col[j] - a[i][j];
    }

    let answer = b
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", answer);
}
