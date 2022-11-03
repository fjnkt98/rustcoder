use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };

    if c == 1 {
        println!("No");
        return;
    }

    for exp in 1..=b {
        if a < c.pow(exp as u32) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
