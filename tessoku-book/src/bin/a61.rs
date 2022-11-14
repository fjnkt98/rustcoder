use proconio::input;

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

    for i in 0..n {
        graph[i].sort();
    }

    println!(
        "{}",
        graph
            .iter()
            .enumerate()
            .map(|(i, node)| {
                format!(
                    "{}: {{{}}}",
                    i + 1,
                    node.iter()
                        .map(|x| (x + 1).to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
