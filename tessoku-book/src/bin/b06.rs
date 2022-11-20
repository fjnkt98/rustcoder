use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let b = a
        .iter()
        .map(|x| if *x == 0 { -1 } else { 1 })
        .cumsum()
        .collect::<Vec<i32>>();
    for (l, r) in lr.iter() {
        let l = *l - 1;
        let r = *r - 1;

        let mut count = b[r];
        if l > 0 {
            count -= b[l - 1];
        }

        match count.cmp(&0) {
            std::cmp::Ordering::Equal => {
                println!("draw");
            }
            std::cmp::Ordering::Greater => {
                println!("win");
            }
            std::cmp::Ordering::Less => {
                println!("lose");
            }
        };
    }
}
