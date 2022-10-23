use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
        m: usize,
        b: [i64; m],
    };

    let mut c = a.iter().cumsum().collect::<Vec<i64>>();
    c.insert(0, 0);

    let mut answer: i64 = 0;
    let b = b.iter().map(|x| x - 1).collect::<Vec<i64>>();
    let mut current = b[0];
    for b in b[1..].iter() {
        let d = if current < *b {
            c[*b as usize] - c[current as usize]
        } else {
            c[current as usize] - c[*b as usize]
        };

        answer += d;
        current = *b;
    }

    println!("{}", answer);
}
