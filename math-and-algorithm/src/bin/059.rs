use proconio::input;

fn main() {
    input! {
        n: i64,
    };

    match n % 4 {
        0 => {
            println!("6")
        }
        1 => {
            println!("2")
        }
        2 => {
            println!("4")
        }
        3 => {
            println!("8")
        }
        _ => {}
    }
}
