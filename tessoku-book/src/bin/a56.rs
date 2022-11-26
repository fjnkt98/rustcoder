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
        while h < 0 {
            h += self.p;
        }
        h %= self.p;

        return h;
    }
}

fn main() {
    input! {
        _: usize,
        q: usize,
        s: String,
        abcd: [(usize, usize, usize, usize); q]
    };

    let p: i64 = 2147483647;

    let hs = HashedString::new(&s, p);
    for &(a, b, c, d) in abcd.iter() {
        let h1 = hs.hash(a, b);
        let h2 = hs.hash(c, d);

        if h1 == h2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
