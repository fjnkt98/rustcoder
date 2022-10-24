extern crate nalgebra as na;
use proconio::input;

fn matrix_multiply(a: &na::Matrix3<i64>, b: &na::Matrix3<i64>) -> na::Matrix3<i64> {
    let p = 1000_000_007;

    let c = a * b;
    return c.map(|x| x % p);
}

fn matrix_power(m: &na::Matrix3<i64>, n: i64) -> na::Matrix3<i64> {
    let mut p: na::Matrix3<i64> = m.clone();
    let mut q: na::Matrix3<i64> = na::Matrix3::zeros();

    let mut flag = false;

    for i in 0..60 {
        if (n & (1 << i)) != 0 {
            if flag {
                q = matrix_multiply(&q, &p);
            } else {
                q = p.clone();
                flag = true;
            }
        }

        p = matrix_multiply(&p, &p);
    }

    return q;
}

fn main() {
    input! {
        n: i64,
    };

    let a: na::Matrix3<i64> = na::Matrix3::new(1, 1, 1, 1, 0, 0, 0, 1, 0);
    let b = matrix_power(&a, n - 1);
    let answer = b.row(2).dot(&na::Vector3::from([2, 1, 1]).transpose()) % 1000000007;
    println!("{}", answer);
}
