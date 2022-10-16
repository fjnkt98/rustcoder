use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    };

    let sum: i32 = a.iter().sum();
    println!("{}", sum % 100);
}
