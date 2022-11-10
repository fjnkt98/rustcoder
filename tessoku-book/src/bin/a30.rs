use proconio::input;

struct BinominalCoefficient {
    n: usize,
    p: i64,
    factorial: Vec<i64>,
    _inverse_element: Vec<i64>,
    inverse_factorial: Vec<i64>,
}

impl BinominalCoefficient {
    pub fn new(n: usize, p: i64) -> BinominalCoefficient {
        assert!(n > 0);
        assert!(p > 0);

        let mut factorial: Vec<i64> = vec![1; n + 1];
        let mut inverse_element: Vec<i64> = vec![1; n + 1];
        let mut inverse_factorial: Vec<i64> = vec![1; n + 1];

        for i in 2..=n {
            factorial[i] = factorial[i - 1] * i as i64 % p;
            inverse_element[i] = p - inverse_element[(p % i as i64) as usize] * (p / i as i64) % p;
            inverse_factorial[i] = inverse_factorial[i - 1] * inverse_element[i] % p;
        }

        return BinominalCoefficient {
            n,
            p,
            factorial,
            _inverse_element: inverse_element,
            inverse_factorial,
        };
    }

    pub fn ncr(&self, n: usize, r: usize) -> i64 {
        assert!(n > 0);
        assert!(n <= self.n);
        assert!(r > 0);
        assert!(r <= self.n);

        return self.factorial[n]
            * (self.inverse_factorial[r] * self.inverse_factorial[n - r] % self.p)
            % self.p;
    }
}

fn main() {
    input! {
        n: usize,
        r: usize,
    };

    let p = 1000000007;

    let bc = BinominalCoefficient::new(n, p);

    println!("{}", bc.ncr(n, r));
}
