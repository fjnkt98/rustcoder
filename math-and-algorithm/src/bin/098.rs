extern crate nalgebra as na;
use std::mem;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
        a: i64,
        b: i64
    };

    xy.push(xy[0].clone());

    let mut count = 0;
    for (pi, pj) in xy.iter().tuple_windows() {
        let (xi, yi) = *pi;
        let (xj, yj) = *pj;

        let mut vi = na::Vector3::new(xi - a, yi - b, 0);
        let mut vj = na::Vector3::new(xj - a, yj - b, 0);

        if vi.cross(&vj).z == 0 && vi.dot(&vj) <= 0 {
            continue;
        }

        if vi.y > vj.y {
            mem::swap(&mut vi, &mut vj);
        }

        if vi.cross(&vj).z > 0 && vi.y <= 0 && vj.y > 0 {
            count += 1;
        }
    }

    if count % 2 == 1 {
        println!("INSIDE");
    } else {
        println!("OUTSIDE");
    }
}
