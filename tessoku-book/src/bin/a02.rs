use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n]
    };

    for &a in a.iter() {
        if x == a {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
