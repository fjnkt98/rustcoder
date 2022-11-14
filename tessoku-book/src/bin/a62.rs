use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, explored: &mut Vec<bool>, current: usize) {
    explored[current] = true;

    for next_node in graph[current].iter() {
        if explored[*next_node] {
            continue;
        }

        dfs(&graph, explored, *next_node);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;

        graph[a].push(b);
        graph[b].push(a);
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
