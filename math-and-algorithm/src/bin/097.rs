use proconio::input;

fn main() {
    input! {
        l: u64,
        r: u64,
    };

    let m = (r as f64).sqrt().ceil() as u64;
    let mut f1 = vec![true; (m + 1) as usize];
    let mut f2 = vec![true; (r - l + 1) as usize];

    if l == 1 {
        f2[0] = false;
    }

    for p in 2..=m {
        if !f1[p as usize] {
            continue;
        }

        let mut q = 2 * p;
        while q * q <= r {
            f1[q as usize] = false;
            q += p;
        }

        let mut s = ((l + p - 1) / p) * p;
        if s == p {
            s = 2 * p;
        }

        while s <= r {
            f2[(s - l) as usize] = false;
            s += p;
        }
    }

    println!("{}", f2.iter().filter(|&x| *x).count());
}
