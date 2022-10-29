use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(i64, i64); n]
    };

    lr.sort_by_key(|x| x.1);

    let mut last = 0;
    let mut answer: i64 = 0;
    for (l, r) in lr.iter() {
        if last <= *l {
            answer += 1;
            last = *r;
        }
    }

    println!("{}", answer);
}
