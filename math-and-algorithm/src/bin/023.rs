use proconio::input;

fn main() {
    input! {
        n: i64,
        b: [i64; n],
        r: [i64; n],
    };

    let mut e1 = 0.0;
    for b in b.iter() {
        e1 += (*b as f64) / (n as f64);
    }

    let mut e2 = 0.0;
    for r in r.iter() {
        e2 += (*r as f64) / (n as f64);
    }

    println!("{}", e1 + e2);
}
