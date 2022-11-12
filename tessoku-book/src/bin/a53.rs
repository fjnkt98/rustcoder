use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let q: usize = line.trim().parse().unwrap();

    let mut query: Vec<String> = vec![String::new(); q];
    for i in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        query[i] = line;
    }

    let mut heap = BinaryHeap::new();
    for s in query.iter() {
        let query = s.trim().split_whitespace().collect::<Vec<&str>>();

        match query[0] {
            "1" => {
                let x: i64 = query[1].parse().unwrap();
                heap.push(cmp::Reverse(x));
            }
            "2" => {
                let y = heap.peek().unwrap();
                println!("{}", y.0);
            }
            "3" => {
                heap.pop();
            }
            _ => {}
        }
    }
}
