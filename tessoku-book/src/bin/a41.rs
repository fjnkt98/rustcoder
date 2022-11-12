use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String
    };

    if s.contains("RRR") || s.contains("BBB") {
        println!("Yes");
    } else {
        println!("No");
    }
}
