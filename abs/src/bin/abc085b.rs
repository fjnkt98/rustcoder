use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        d: [i32; n],
    };

    let uniq: HashSet<i32> = d.into_iter().collect();
    let d: Vec<i32> = uniq.into_iter().collect();

    println!("{}", d.len());
}
