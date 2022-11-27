use proconio::input;

struct HashedString {
    x: Vec<i64>,
    h: Vec<i64>,
    p: i64,
}

impl HashedString {
    fn new(s: &str, p: i64) -> HashedString {
        let s = s.chars().map(|c| c as i64 - 48).collect::<Vec<i64>>();
        let n = s.len();

        let mut x: Vec<i64> = vec![1; n + 1];
        let mut h: Vec<i64> = vec![0; n + 1];
        for i in 1..=n {
            x[i] = (100 * x[i - 1]) % p;

            h[i] = h[i - 1] * 100 + s[i - 1];
            h[i] %= p;
        }

        return HashedString { x, h, p };
    }

    fn hash(&self, l: usize, r: usize) -> i64 {
        let mut h = self.h[r] - self.x[r - l + 1] * self.h[l - 1];
        if h < 0 {
            h += self.p.pow(2);
        }
        return h % self.p;
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lr: [(usize, usize); q]
    };

    let p: i64 = 2147483647;

    let t = s.chars().rev().collect::<String>();

    let hs = HashedString::new(&s, p);
    let ht = HashedString::new(&t, p);

    for &(l, r) in lr.iter() {
        let rl = n - r + 1;
        let rr = n - l + 1;

        if hs.hash(l, r) == ht.hash(rl, rr) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
