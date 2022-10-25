use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; k]
    };

    let mut answer = 0;
    for bits in 0..(1 << k) {
        let mut lcm = 1;
        let mut count = 0;
        for i in 0..k {
            if (bits >> i) & 1 == 1 {
                lcm = lcm.lcm(&a[i as usize]);
                count += 1
            }
        }

        if count % 2 == 0 {
            answer += n / lcm;
        } else {
            answer -= n / lcm;
        }
    }

    println!("{}", n - answer);
}
