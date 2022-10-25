use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    for i in 1..63i64 {
        if (1i64 << i) - 1 == n {
            println!("Second");
            return;
        }
    }

    println!("First");
}
