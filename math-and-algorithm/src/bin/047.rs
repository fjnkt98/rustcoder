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
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut colors = vec![-1; n];

    let mut candidate = VecDeque::new();

    for i in 0..n {
        if colors[i] != -1 {
            continue;
        }

        colors[i] = 0;
        candidate.push_back((i, 0));

        while let Some((node, color)) = candidate.pop_front() {
            for next_node in graph[node].iter() {
                if colors[*next_node] == -1 {
                    colors[*next_node] = 1 - color;
                    candidate.push_back((*next_node, 1 - color));
                } else if colors[*next_node] == color {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
