use proconio::input;

fn f(x: f64) -> f64 {
    return x.powf(3.0) + x;
}

fn main() {
    input! {
        n: f64,
    };

    let mut left = 0.0;
    let mut right = 1e10;
    while right - left > 1e-6 {
        let mid = (right + left) / 2.0;

        if f(mid) > n {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", left);
}
