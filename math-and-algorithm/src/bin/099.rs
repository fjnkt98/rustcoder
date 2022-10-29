use proconio::input;
use std::cmp;

fn dfs(graph: &Vec<Vec<usize>>, dp: &mut Vec<i64>, current: usize, previous: usize) {
    dp[current] = 1;

    for &next_node in graph[current].iter() {
        if next_node == previous {
            continue;
        }

        dfs(graph, dp, next_node, current);
        dp[current] += dp[next_node];
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for &(a, b) in ab.iter() {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut dp: Vec<i64> = vec![1 << 60; n];

    dfs(&graph, &mut dp, 0, 0);

    let mut answer = 0;
    for &(a, b) in ab.iter() {
        let r = cmp::min(dp[a - 1], dp[b - 1]);
        answer += r * (n as i64 - r);
    }

    println!("{}", answer);
}
