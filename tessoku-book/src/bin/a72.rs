use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i64,
        c: [String; h]
    };

    let grid = c
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut answer = 0;
    'bits: for bits in 0..1i64 << h {
        let mut grid = grid.clone();

        let mut bit_count = 0;
        for i in 0..h {
            if (bits >> i) & 1 == 1 {
                bit_count += 1;
            }
        }
        if bit_count > k {
            continue 'bits;
        }

        let mut remain = k;
        for i in 0..h {
            if (bits >> i) & 1 == 1 {
                for j in 0..w {
                    grid[i][j] = '#';
                }
                remain -= 1;
            }
        }

        let mut count = vec![0; w];
        for (i, j) in iproduct!(0..h, 0..w) {
            if grid[i][j] == '.' {
                count[j] += 1;
            }
        }

        let mut column: Vec<(usize, i64)> = Vec::new();
        for (col, count) in count.iter().enumerate() {
            column.push((col, *count));
        }
        column.sort_by_key(|x| std::cmp::Reverse(x.1));
        for &(col, _) in column[0..remain as usize].iter() {
            for i in 0..h {
                grid[i][col] = '#';
            }
        }

        let count: i64 = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c == '#' { 1 } else { 0 })
                    .sum::<i64>()
            })
            .sum::<i64>();

        answer = std::cmp::max(answer, count);
    }

    println!("{}", answer);
}
