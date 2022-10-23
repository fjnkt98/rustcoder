use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut c = vec![0; t + 1];
    for (l, r) in lr.iter() {
        c[*l] += 1;
        c[*r] -= 1;
    }

    c = c.iter().cumsum().collect::<Vec<i64>>();
    c.pop();
    let c = c.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    println!("{}", c.join("\n"));
}
