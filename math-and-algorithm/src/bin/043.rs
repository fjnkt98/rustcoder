use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, explored: &mut Vec<bool>, current_node: usize) {
    explored[current_node] = true;

    for next_node in graph[current_node].iter() {
        if explored[*next_node] {
            continue;
        }

        dfs(graph, explored, *next_node);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
    for (a, b) in ab.iter() {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut explored = vec![false; n];
    explored[0] = true;

    dfs(&graph, &mut explored, 0);

    if explored.iter().all(|x| *x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}
