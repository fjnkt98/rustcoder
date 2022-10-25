use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h]
    };

    let mut row_sum = vec![0i64; h];
    let mut col_sum = vec![0i64; w];
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j];
        }
    }

    let mut b = vec![vec![0i64; w]; h];
    for i in 0..h {
        for j in 0..w {
            b[i][j] = row_sum[i] + col_sum[j] - a[i][j];
        }
    }

    println!(
        "{}",
        b.iter()
            .map(|r| {
                r.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
