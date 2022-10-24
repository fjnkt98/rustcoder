use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    };

    let z = x.abs() + y.abs();
    if z <= n && z % 2 == n % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
