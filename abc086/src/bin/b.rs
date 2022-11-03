use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    };

    let c: i64 = (a + &b).parse().unwrap();
    for i in 1..=1000 {
        if c == i * i {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
