use proconio::input;
use std::collections::BinaryHeap;

struct Dijkstra<'a> {
    n: usize,
    graph: &'a Vec<Vec<(usize, i64)>>,
    distance: Vec<i64>,
    previous: Vec<i64>,
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Vec<Vec<(usize, i64)>>) -> Dijkstra {
        let n = graph.len();
        let distance: Vec<i64> = vec![1 << 60; n];
        let previous: Vec<i64> = vec![-1; n];

        return Dijkstra {
            n: graph.len(),
            graph: graph,
            distance: distance,
            previous: previous,
        };
    }

    pub fn search(&'a mut self, start: usize) -> &'a Vec<i64> {
        self.distance = vec![1 << 60; self.n];
        self.previous = vec![-1; self.n];

        let mut candidate: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        candidate.push((0, start));

        self.distance[start] = 0;

        while let Some((mut d, node)) = candidate.pop() {
            d = -d;
            if d > self.distance[node] {
                continue;
            }

            for (next_node, weight) in self.graph[node].iter() {
                if self.distance[*next_node] > self.distance[node] + *weight {
                    self.distance[*next_node] = self.distance[node] + *weight;
                    self.previous[*next_node] = node as i64;
                    candidate.push((-self.distance[*next_node], *next_node));
                }
            }
        }

        return &self.distance;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m]
    };

    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for (a, b, c) in abc.iter() {
        graph[*a - 1].push((*b - 1, *c));
        graph[*b - 1].push((*a - 1, *c));
    }

    let mut dijkstra = Dijkstra::new(&graph);
    let distance = dijkstra.search(0);

    if distance[n - 1] == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", distance[n - 1]);
    }
}
