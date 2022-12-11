use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    };

    let max = h.iter().max().unwrap();
    println!("{}", h.iter().position(|x| x == max).unwrap() + 1);
}
