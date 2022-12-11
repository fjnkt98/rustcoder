use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [String; h],
    };

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut xs = vec![0; w];
    for (i, j) in iproduct!(0..h, 0..w) {
        if grid[i][j] == '#' {
            xs[j] += 1;
        }
    }

    println!(
        "{}",
        xs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
