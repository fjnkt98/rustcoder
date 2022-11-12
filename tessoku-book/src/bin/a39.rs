use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };

    let mut answer = 0;
    let mut last = 0;

    lr.sort_by_key(|x| x.1);
    for &(l, r) in lr.iter() {
        if last <= l {
            answer += 1;
            last = r;
        }
    }

    println!("{}", answer);
}
