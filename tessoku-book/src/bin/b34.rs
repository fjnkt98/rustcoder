use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        a: [usize; n]
    };

    let g = [0, 0, 1, 1, 2];
    let mut sum = 0;
    for &a in a.iter() {
        sum ^= g[a % 5];
    }

    if sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
