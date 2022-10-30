use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: i64,
        mut ab: [(i64, i64); n]
    };

    ab.sort();
    let mut answer = 0;
    for &(a, b) in ab.iter() {
        if m >= b {
            answer += a * b;
            m -= b;
        } else {
            answer += a * m;
            m = 0;
        }

        if m == 0 {
            break;
        }
    }

    println!("{}", answer);
}
