use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        k: i64,
        a: [i64; n]
    };

    let mut left: i64 = 0;
    let mut right: i64 = l + 1;
    while right - left > 1 {
        let mid = (right + left) / 2;

        let mut count: i64 = 0;
        let mut last_cut: i64 = 0;

        for &a in a.iter() {
            if a - last_cut >= mid && l - a >= mid {
                last_cut = a;
                count += 1;
            }
        }

        if count >= k {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
