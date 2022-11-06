use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    };

    let mut grid = vec![vec![0i64; w + 1]; h + 1];
    for &(a, b, c, d) in abcd.iter() {
        let a = a - 1;
        let b = b - 1;
        let c = c - 1;
        let d = d - 1;

        grid[a][b] += 1; // 左上
        grid[a][d + 1] -= 1; // 右上
        grid[c + 1][b] -= 1; // 左下
        grid[c + 1][d + 1] += 1; // 右下
    }

    for i in 0..=h {
        for j in 1..=w {
            grid[i][j] += grid[i][j - 1];
        }
    }
    for j in 0..=w {
        for i in 1..=h {
            grid[i][j] += grid[i - 1][j];
        }
    }

    println!(
        "{}",
        grid.iter()
            .map(|r| r[0..w]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()[0..h]
            .join("\n")
    )
}
