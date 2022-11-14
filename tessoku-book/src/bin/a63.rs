use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b) in ab.iter() {
        let a = *a - 1;
        let b = *b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut distance = vec![-1; n];
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

    println!(
        "{}",
        distance
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
