use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, grade: &mut Vec<i64>, current: usize, previous: usize) -> i64 {
    let mut max_grade = -1;
    for &next_node in graph[current].iter() {
        if next_node == previous {
            continue;
        }
        max_grade = max_grade.max(dfs(graph, grade, next_node, current));
    }
    grade[current] = max_grade + 1;
    return grade[current];
}

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, usize); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for (a, b) in ab.iter() {
        let a = *a - 1;
        let b = *b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut grade = vec![0; n];
    dfs(&graph, &mut grade, t - 1, 1 << 60);
    println!(
        "{}",
        grade
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
