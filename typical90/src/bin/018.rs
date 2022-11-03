use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x_t: f64,
        y_t: f64,
        q: usize,
        e: [f64; q]
    };

    for &e in e.iter() {
        let theta = 2.0 * std::f64::consts::PI * e / t;

        let y = -l * theta.sin() / 2.0;
        let z = -l * theta.cos() / 2.0 + l / 2.0;

        let d = (x_t.powf(2.0) + (y - y_t).powf(2.0)).sqrt();
        let answer = (z / d).atan();

        println!("{}", answer.to_degrees());
    }
}
