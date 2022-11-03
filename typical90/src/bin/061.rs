use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(u8, i64); q]
    };

    let mut deq: VecDeque<i64> = VecDeque::new();
    for &(t, x) in tx.iter() {
        match t {
            1 => deq.push_front(x),
            2 => deq.push_back(x),
            3 => {
                println!("{}", deq[(x - 1) as usize])
            }
            _ => {}
        }
    }
}
