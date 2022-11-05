use proconio::input;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        txy: [(u8, usize, usize); q]
    };

    let mut deque: VecDeque<i64> = VecDeque::from_iter(a);
    for &(t, x, y) in txy.iter() {
        match t {
            1 => deque.swap(x - 1, y - 1),
            2 => deque.rotate_right(1),
            3 => {
                println!("{}", deque[x - 1])
            }
            _ => {}
        }
    }
}
