use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        mut a: [i64; n]
    };

    a.sort();

    if binary_search(&a, x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn binary_search<T>(a: &[T], x: T) -> bool
where
    T: PartialOrd,
{
    let mut left: usize = 0;
    let mut right: usize = a.len();

    while right - left > 1 {
        let mid = (right + left) / 2;

        if a[mid] == x {
            return true;
        } else if a[mid] > x {
            right = mid;
        } else {
            left = mid;
        }
    }

    return a[left] == x;
}
