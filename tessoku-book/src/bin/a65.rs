use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, count: &mut Vec<i64>, current: usize, previous: usize) -> i64 {
    for &next_node in graph[current].iter() {
        if next_node == previous {
            continue;
        }
        count[current] += dfs(&graph, count, next_node, current);
    }

    return count[current] + 1;
}

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1]
    };

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, a) in a.iter().enumerate() {
        let a = *a - 1;
        graph[i + 1].push(a);
        graph[a].push(i + 1);
    }

    let mut count = vec![0; n];
    dfs(&graph, &mut count, 0, 1 << 30);

    println!(
        "{}",
        count
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
