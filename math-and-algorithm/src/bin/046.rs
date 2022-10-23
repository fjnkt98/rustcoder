use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (usize, usize),
        g: (usize, usize),
        grid: [String; h]
    };

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut distance: Vec<Vec<i64>> = vec![vec![-1; w]; h];
    let (sr, sc) = s;
    distance[sr - 1][sc - 1] = 0;
    let mut candidate: VecDeque<(i64, i64)> = VecDeque::new();
    candidate.push_back(((sr - 1) as i64, (sc - 1) as i64));

    let dr = [0, 1, 0, -1];
    let dc = [1, 0, -1, 0];

    while let Some((r, c)) = candidate.pop_front() {
        for i in 0..4 {
            let nr = r + dr[i];
            let nc = c + dc[i];

            if 0 <= nr
                && nr < h as i64
                && 0 <= nc
                && nc < w as i64
                && grid[nr as usize][nc as usize] != '#'
                && distance[nr as usize][nc as usize] == -1
            {
                distance[nr as usize][nc as usize] = distance[r as usize][c as usize] + 1;
                candidate.push_back((nr, nc));
            }
        }
    }

    let (gr, gc) = g;

    println!("{}", distance[gr - 1][gc - 1]);
}
