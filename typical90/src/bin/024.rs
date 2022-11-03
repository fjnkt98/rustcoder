use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    };

    let mut diff = 0;
    for (&a, &b) in a.iter().zip(b.iter()) {
        diff += (a - b).abs();
    }

    if k >= diff && k % 2 == diff % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
