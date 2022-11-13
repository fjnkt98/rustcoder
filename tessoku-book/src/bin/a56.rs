use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        abcd: [(usize, usize, usize, usize); q]
    };

    let p: i64 = 2147483647;
    let s = s.chars().map(|c| c as i64 - 48).collect::<Vec<i64>>();
    let mut x: Vec<i64> = vec![1; n + 1];
    let mut h: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        x[i] = (100 * x[i - 1]) % p;

        h[i] = h[i - 1] * 100 + s[i - 1];
        h[i] %= p;
    }

    for &(a, b, c, d) in abcd.iter() {
        let a = a - 1;
        let c = c - 1;

        let mut h1 = h[b] - x[b - a] * h[a];
        while h1 < 0 {
            h1 += p;
        }
        h1 %= p;
        let mut h2 = h[d] - x[d - c] * h[c];
        while h2 < 0 {
            h2 += p;
        }
        h2 %= p;

        if h1 == h2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
