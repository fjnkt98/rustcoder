use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: isize,
        w: isize,
        s: (isize, isize),
        g: (isize, isize),
        grid: [String; h]
    };

    let grid = grid
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let sr = s.0 - 1;
    let sc = s.1 - 1;
    let gr = g.0 - 1;
    let gc = g.1 - 1;

    let mut candidate: VecDeque<(isize, isize)> = VecDeque::new();
    candidate.push_back((sr, sc));
    let mut distance: Vec<Vec<i64>> = vec![vec![1 << 60; w as usize]; h as usize];
    distance[sr as usize][sc as usize] = 0;

    let dr = [0, 1, 0, -1];
    let dc = [1, 0, -1, 0];
    while let Some((r, c)) = candidate.pop_front() {
        for i in 0..4 {
            let nr = r + dr[i];
            let nc = c + dc[i];

            if 0 <= nr
                && nr < h
                && 0 <= nc
                && nc < w
                && distance[nr as usize][nc as usize] == 1 << 60
                && grid[nr as usize][nc as usize] == '.'
            {
                distance[nr as usize][nc as usize] = distance[r as usize][c as usize] + 1;
                candidate.push_back((nr, nc));
            }
        }
    }

    println!("{}", distance[gr as usize][gc as usize]);
}
