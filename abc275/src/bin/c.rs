use itertools::iproduct;
use proconio::input;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Square {
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
    p4: (usize, usize),
}

fn is_inbound(p: (i64, i64)) -> bool {
    return 0 <= p.0 && p.0 < 9 && 0 <= p.1 && p.1 < 9;
}

fn main() {
    input! {
        grid: [String; 9],
    };

    let grid = grid
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pawn: Vec<(i64, i64)> = Vec::new();
    for (i, j) in iproduct!(0..9, 0..9) {
        if grid[i][j] == '#' {
            pawn.push((i as i64, j as i64));
        }
    }

    let mut squares: HashSet<Square> = HashSet::new();
    for i in 0..pawn.len() {
        for j in i + 1..pawn.len() {
            let r1 = pawn[i].0;
            let c1 = pawn[i].1;

            let r2 = pawn[j].0;
            let c2 = pawn[j].1;

            let r_diff = r2 - r1;
            let c_diff = c2 - c1;

            let r3 = r1 + c_diff;
            let c3 = c1 - r_diff;

            let r4 = r2 + c_diff;
            let c4 = c2 - r_diff;

            if is_inbound((r3, c3))
                && is_inbound((r4, c4))
                && grid[r3 as usize][c3 as usize] == '#'
                && grid[r4 as usize][c4 as usize] == '#'
            {
                let mut vertices = vec![
                    (r1 as usize, c1 as usize),
                    (r2 as usize, c2 as usize),
                    (r3 as usize, c3 as usize),
                    (r4 as usize, c4 as usize),
                ];
                vertices.sort();

                let square = Square {
                    p1: vertices[0],
                    p2: vertices[1],
                    p3: vertices[2],
                    p4: vertices[3],
                };

                squares.insert(square);
            }

            let r3 = r1 - c_diff;
            let c3 = c1 + r_diff;

            let r4 = r2 - c_diff;
            let c4 = c2 + r_diff;

            if is_inbound((r3, c3))
                && is_inbound((r4, c4))
                && grid[r3 as usize][c3 as usize] == '#'
                && grid[r4 as usize][c4 as usize] == '#'
            {
                let mut vertices = vec![
                    (r1 as usize, c1 as usize),
                    (r2 as usize, c2 as usize),
                    (r3 as usize, c3 as usize),
                    (r4 as usize, c4 as usize),
                ];
                vertices.sort();

                let square = Square {
                    p1: vertices[0],
                    p2: vertices[1],
                    p3: vertices[2],
                    p4: vertices[3],
                };

                squares.insert(square);
            }
        }
    }
    println!("{}", squares.len());
    // for square in squares.iter() {
    //     println!("{:?}", square);
    // }
}
