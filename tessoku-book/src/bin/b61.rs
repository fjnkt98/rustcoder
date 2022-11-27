use proconio::input;

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

    let mut count = 0;
    let mut answer = 0;
    for (i, node) in graph.iter().enumerate() {
        if node.len() > count {
            count = node.len();
            answer = i;
        }
    }

    println!("{}", answer + 1);
}
