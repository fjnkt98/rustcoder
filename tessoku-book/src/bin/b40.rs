use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    let mut counter: HashMap<i64, i64> = HashMap::new();
    for &a in a.iter() {
        *counter.entry(a % 100).or_insert(0) += 1;
    }

    let mut answer = 0;
    for i in 0i64..=50 {
        match i {
            0 => {
                let c = counter.get(&0).unwrap_or(&0);
                answer += c * (c - 1) / 2;
            }
            50 => {
                let c = counter.get(&50).unwrap_or(&0);
                answer += c * (c - 1) / 2;
            }
            _ => {
                let a = counter.get(&i).unwrap_or(&0);
                let b = counter.get(&(100 - i)).unwrap_or(&0);

                answer += a * b;
            }
        }
    }

    println!("{}", answer);
}
