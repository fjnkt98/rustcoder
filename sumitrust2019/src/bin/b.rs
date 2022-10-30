use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    for i in 0..=n {
        if i * 108 / 100 == n {
            println!("{}", i);
            return;
        }
    }

    println!(":(");
}
