use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    };

    let mut answer: i64 = 0;
    for a in a.iter().combinations(5) {
        let sum: i64 = a.iter().fold(0, |sum, x| sum + **x);
        if sum == 1000 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
