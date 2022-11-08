use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    };

    let mut left = 0;
    let mut right = 2000000000;
    while right - left > 1 {
        let mid = (right + left) >> 1;

        let mut count = 0;
        for &a in a.iter() {
            count += mid / a;
        }

        if count >= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
