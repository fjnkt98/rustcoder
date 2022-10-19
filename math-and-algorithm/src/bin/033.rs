extern crate nalgebra as na;
use proconio::input;

fn main() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    };

    let ba = na::Vector3::new(a.0 - b.0, a.1 - b.1, 0.0);
    let bc = na::Vector3::new(c.0 - b.0, c.1 - b.1, 0.0);

    let ca = na::Vector3::new(a.0 - c.0, a.1 - c.1, 0.0);
    let cb = na::Vector3::new(b.0 - c.0, b.1 - c.1, 0.0);

    if ba.dot(&bc) < 0.0 {
        println!("{}", ba.norm());
    } else if ca.dot(&cb) < 0.0 {
        println!("{}", ca.norm());
    } else {
        println!("{}", ba.cross(&bc).norm().abs() / cb.norm());
    }
}
