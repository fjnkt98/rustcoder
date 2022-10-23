extern crate nalgebra as na;
use proconio::input;
use std::cmp;
use std::mem;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
        x4: i64,
        y4: i64,
    };

    if (y3 - y1) * (x1 - x2) == (y1 - y2) * (x3 - x1)
        && (y4 - y1) * (x1 - x2) == (y1 - y2) * (x4 - x1)
    {
        let mut a = (x1, y1);
        let mut b = (x2, y2);
        let mut c = (x3, y3);
        let mut d = (x4, y4);

        if a > b {
            mem::swap(&mut a, &mut b);
        }

        if c > d {
            mem::swap(&mut c, &mut d);
        }

        if cmp::max(a, c) <= cmp::min(b, d) {
            println!("Yes");
        } else {
            println!("No");
        }

        return;
    }

    let x12 = na::Vector3::new(x2 - x1, y2 - y1, 0);
    let x13 = na::Vector3::new(x3 - x1, y3 - y1, 0);
    let x14 = na::Vector3::new(x4 - x1, y4 - y1, 0);

    let intersect1: bool = (x12.cross(&x14).z >= 0 && x12.cross(&x13).z <= 0)
        || (x12.cross(&x14).z <= 0 && x12.cross(&x13).z >= 0);

    let x43 = na::Vector3::new(x3 - x4, y3 - y4, 0);
    let x41 = na::Vector3::new(x1 - x4, y1 - y4, 0);
    let x42 = na::Vector3::new(x2 - x4, y2 - y4, 0);

    let intersect2: bool = (x43.cross(&x42).z >= 0 && x43.cross(&x41).z <= 0)
        || (x43.cross(&x42).z <= 0 && x43.cross(&x41).z >= 0);

    if intersect1 && intersect2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
