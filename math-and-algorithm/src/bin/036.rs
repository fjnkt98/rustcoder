extern crate nalgebra as na;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        h: i64,
        m: i64,
    };

    let theta_a: f64 = ((m * 2) as f64) * std::f64::consts::PI / (60 as f64);
    let theta_b: f64 = (h * 2) as f64 * std::f64::consts::PI / 12 as f64
        + (m * 2) as f64 * std::f64::consts::PI / (12 * 60) as f64;

    let rot_a = na::Matrix2::new(theta_a.cos(), -theta_a.sin(), theta_a.sin(), theta_a.cos());
    let rot_b = na::Matrix2::new(theta_b.cos(), -theta_b.sin(), theta_b.sin(), theta_b.cos());

    let mut a = na::Vector2::new(a as f64, 0.0);
    let mut b = na::Vector2::new(b as f64, 0.0);

    a = rot_a * a;
    b = rot_b * b;

    let l = (a - b).norm();

    println!("{}", l);
}
