use proconio::input;

fn dfs(
    graph: &Vec<Vec<usize>>,
    explored: &mut Vec<bool>,
    history: &mut Vec<usize>,
    current: usize,
    goal: usize,
) {
    explored[current] = true;

    if current == goal {
        println!(
            "{}",
            history
                .iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        for i in 0..explored.len() {
            explored[i] = true;
        }
        return;
    }

    for &next_node in graph[current].iter() {
        if explored[next_node] {
            continue;
        }
        history.push(next_node);
        dfs(graph, explored, history, next_node, goal);
        history.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (a, b) in ab.iter() {
        let a = *a - 1;
        let b = *b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut explored = vec![false; n];
    let mut history = Vec::new();
    history.push(0);
    dfs(&graph, &mut explored, &mut history, 0, n - 1);
}
