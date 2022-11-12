use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(i64, i64); n]
    };

    let mut answer = 0;

    for i in 0..=100 {
        for j in 0..=100 {
            let mut count = 0;
            for &(a, b) in ab.iter() {
                if (i <= a && a <= i + k) && (j <= b && b <= j + k) {
                    count += 1;
                }
            }

            answer = std::cmp::max(answer, count);
        }
    }

    println!("{}", answer);
}
