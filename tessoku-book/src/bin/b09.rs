use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    };

    let mut grid: Vec<Vec<i64>> = vec![vec![0; 1502]; 1502];
    for &(a, b, c, d) in abcd.iter() {
        grid[a][b] += 1;
        grid[a][d] -= 1;
        grid[c][b] -= 1;
        grid[c][d] += 1;
    }

    for (i, j) in iproduct!(0..=1501, 1..=1501) {
        grid[i][j] += grid[i][j - 1];
    }
    for (i, j) in iproduct!(1..=1501, 0..=1501) {
        grid[i][j] += grid[i - 1][j];
    }

    let mut answer: i64 = 0;
    for (i, j) in iproduct!(0..=1501, 0..=1501) {
        if grid[i][j] > 0 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
