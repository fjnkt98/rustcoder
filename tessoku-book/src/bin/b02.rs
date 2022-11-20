use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    for x in a..=b {
        if 100 % x == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
