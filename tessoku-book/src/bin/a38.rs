use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, i64); n]
    };

    let mut a = vec![24; d + 1];
    a[0] = 0;
    for &(l, r, h) in lrh.iter() {
        for i in l..=r {
            a[i] = std::cmp::min(a[i], h);
        }
    }

    println!("{}", a.iter().sum::<i64>());
}
