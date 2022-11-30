use proconio::input;
use std::collections::{BTreeSet, HashMap};

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        n: usize,
        rc: [(usize, usize); n],
        q: usize,
        dl: [(char, usize); q],
    };

    let mut rs = rs - 1;
    let mut cs = cs - 1;
    let mut rows: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    let mut cols: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    for (r, c) in rc.iter() {
        let r = *r - 1;
        let c = *c - 1;

        let e = rows.entry(r).or_insert(BTreeSet::new());
        e.insert(c);
        let e = cols.entry(c).or_insert(BTreeSet::new());
        e.insert(r);
    }

    for &(d, l) in dl.iter() {
        match d {
            'L' => {
                let left = if cs < l { 0 } else { cs - l };
                let right = cs;
                if let Some(row) = rows.get(&rs) {
                    let blocks = row.range(left..right);

                    if let Some(nc) = blocks.last() {
                        cs = *nc + 1;
                    } else {
                        cs = left;
                    }
                } else {
                    cs = left;
                }
            }
            'R' => {
                let left = cs + 1;
                let right = cs + l + 1;
                if let Some(row) = rows.get(&rs) {
                    let mut blocks = row.range(left..right);

                    if let Some(nc) = blocks.next() {
                        cs = *nc - 1;
                    } else {
                        cs = std::cmp::min(cs + l, w - 1);
                    }
                } else {
                    cs = std::cmp::min(cs + l, w - 1);
                }
            }
            'U' => {
                let bottom = if rs < l { 0 } else { rs - l };
                let top = rs;
                if let Some(col) = cols.get(&cs) {
                    let blocks = col.range(bottom..top);

                    if let Some(nr) = blocks.last() {
                        rs = *nr + 1;
                    } else {
                        rs = bottom;
                    }
                } else {
                    rs = bottom;
                }
            }
            'D' => {
                let bottom = rs + 1;
                let top = rs + l + 1;
                if let Some(col) = cols.get(&cs) {
                    let mut blocks = col.range(bottom..top);

                    if let Some(nr) = blocks.next() {
                        rs = *nr - 1;
                    } else {
                        rs = std::cmp::min(h - 1, rs + l);
                    }
                } else {
                    rs = std::cmp::min(h - 1, rs + l);
                }
            }
            _ => {}
        }
        println!("{} {}", rs + 1, cs + 1);
    }
}
