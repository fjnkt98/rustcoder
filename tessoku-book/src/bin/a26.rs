use proconio::input;

#[allow(dead_code)]
struct EratosthenesSieve {
    n: usize,
    is_prime: Vec<bool>,
    primes: Vec<i64>,
    minimum_factor_of: Vec<i64>,
}

#[allow(dead_code)]
impl<'a> EratosthenesSieve {
    pub fn new(n: usize) -> EratosthenesSieve {
        assert!(n > 0);

        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut primes: Vec<i64> = Vec::with_capacity(n + 1);
        let mut minimum_factor_of: Vec<i64> = vec![-1; n + 1];
        minimum_factor_of[1] = 1;

        for i in 2..=n {
            if !is_prime[i] {
                continue;
            }

            primes.push(i as i64);
            minimum_factor_of[i] = i as i64;

            for j in (2 * i..=n).step_by(i) {
                is_prime[j] = false;

                if minimum_factor_of[j] == -1 {
                    minimum_factor_of[j] = i as i64;
                }
            }
        }

        return EratosthenesSieve {
            n,
            is_prime,
            primes,
            minimum_factor_of,
        };
    }

    pub fn factorize(&self, n: usize) -> Vec<(i64, i64)> {
        assert!(self.n <= n);

        let mut result: Vec<(i64, i64)> = Vec::new();
        let mut n = n;
        while n > 1 {
            let p = self.minimum_factor_of[n];
            let mut exp: i64 = 0;

            while self.minimum_factor_of[n] == p {
                n /= p as usize;
                exp += 1;
            }

            result.push((p, exp));
        }

        return result;
    }

    pub fn divisors(&self, n: usize) -> Vec<i64> {
        assert!(self.n <= n);

        let mut result: Vec<i64> = Vec::new();
        let factors = self.factorize(n);

        for (p, exp) in factors {
            let size = result.len();
            for i in 0..size {
                let mut v = 1;
                for _ in 0..exp {
                    v *= p;
                    result.push(result[i] * v);
                }
            }
        }

        result.sort();

        return result;
    }

    pub fn primes(&'a self) -> &'a [i64] {
        return &self.primes;
    }

    pub fn is_prime(&'a self) -> &'a [bool] {
        return &self.is_prime;
    }
}

fn main() {
    input! {
        q: usize,
        x: [usize; q]
    };

    let es = EratosthenesSieve::new(*x.iter().max().unwrap());

    let is_prime = es.is_prime();
    for &x in x.iter() {
        if is_prime[x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
