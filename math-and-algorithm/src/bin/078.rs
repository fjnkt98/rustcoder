use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n / 2); n];
    for (a, b) in ab.iter() {
        graph[*a - 1].push(*b - 1);
        graph[*b - 1].push(*a - 1);
    }

    let mut distance: Vec<i64> = vec![-1; n];
    distance[0] = 0;
    let mut candidate: VecDeque<usize> = VecDeque::new();
    candidate.push_back(0);

    while let Some(node) = candidate.pop_front() {
        for next_node in graph[node].iter() {
            if distance[*next_node] == -1 {
                distance[*next_node] = distance[node] + 1;
                candidate.push_back(*next_node);
            }
        }
    }

    for d in distance.iter() {
        if *d > 120 || *d == -1 {
            println!("120");
        } else {
            println!("{}", d);
        }
    }
}
