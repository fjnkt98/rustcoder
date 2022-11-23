use proconio::input;

fn main() {
    input! {
        _: usize,
        k: usize,
        s: String
    };

    let count = s.chars().filter(|&x| x == '1').count();
    if count % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
