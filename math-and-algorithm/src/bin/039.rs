use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(i64, i64, i64); q]
    };

    let mut c = vec![0; n + 2];
    for (l, r, x) in lrx.iter() {
        c[*l as usize] += x;
        c[(r + 1) as usize] -= x;
    }

    c = c.iter().cumsum().collect::<Vec<i64>>();

    let mut answer = String::new();
    for i in 1..n {
        if c[i] > c[i + 1] {
            answer.push('>');
        } else if c[i] == c[i + 1] {
            answer.push('=');
        } else {
            answer.push('<');
        }
    }

    println!("{}", answer);
}
