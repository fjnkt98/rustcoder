use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m]
    };

    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for (a, b, c) in abc.iter() {
        let a = *a - 1;
        let b = *b - 1;

        graph[a].push((b, *c));
        graph[b].push((a, *c));
    }

    let mut distance: Vec<i64> = vec![1 << 60; n];
    distance[0] = 0;
    let mut candidate: VecDeque<(i64, usize)> = VecDeque::new();
    candidate.push_back((0, 0));

    while let Some((d, node)) = candidate.pop_front() {
        if d > distance[node] {
            continue;
        }

        for &(next_node, weight) in graph[node].iter() {
            if distance[next_node] > distance[node] + weight {
                distance[next_node] = distance[node] + weight;
                candidate.push_back((distance[next_node], next_node));
            }
        }
    }

    println!(
        "{}",
        distance
            .iter()
            .map(|x| if *x == 1 << 60 {
                String::from("-1")
            } else {
                x.to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
