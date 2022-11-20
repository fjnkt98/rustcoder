use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u8; n],
        xyz: [(usize, usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); 1usize << n];
    for from in 0..1 << n {
        for (x, y, z) in xyz.iter() {
            let (x, y, z) = (*x - 1, *y - 1, *z - 1);

            let mut to = from;
            to ^= 1 << x;
            to ^= 1 << y;
            to ^= 1 << z;

            graph[from].push(to);
        }
    }

    let mut start = 0;
    for (i, a) in a.iter().enumerate() {
        if *a == 1 {
            start |= 1 << i;
        }
    }

    let mut distance: Vec<i64> = vec![-1; 1usize << n];
    distance[start] = 0;
    let mut candidate = VecDeque::new();
    candidate.push_back(start);

    while let Some(node) = candidate.pop_front() {
        for &next_node in graph[node].iter() {
            if distance[next_node] == -1 {
                distance[next_node] = distance[node] + 1;
                candidate.push_back(next_node);
            }
        }
    }

    println!("{}", distance[(1 << n) - 1]);
}
