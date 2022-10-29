use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i64,
        b: i64,
    };

    let mut answer = 0;

    for k in 0..=n.to_string().len() {
        for cmb in (1..=9).combinations_with_replacement(k) {
            let m: i64 = cmb.iter().product::<i64>() + b;
            if m > n {
                continue;
            }

            let mut count1 = vec![0; 10];
            for &c in cmb.iter() {
                count1[c as usize] += 1;
            }

            let mut count2 = vec![0; 10];
            for c in m.to_string().chars() {
                count2[c.to_digit(10).unwrap() as usize] += 1;
            }

            if count1 == count2 {
                answer += 1;
            }
        }
    }

    if n >= b && b.to_string().contains("0") {
        answer += 1;
    }

    println!("{}", answer);
}
