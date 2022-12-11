use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    };

    println!("{:.3}", (1000.0 * b / a).round() / 1000.0);
}
