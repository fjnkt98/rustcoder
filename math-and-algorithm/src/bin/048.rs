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

    pub fn restore_path(&self, end: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        let mut t = end as i64;

        while t != -1 {
            path.push(t as usize);
            t = self.previous[t as usize];
        }

        return path;
    }
}

fn main() {
    input! {
        k: i64,
    };

    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); (k + 1) as usize];
    if k < 10 {
        for i in 1..k {
            graph[k as usize].push(((i % k) as usize, i as i64));
        }
    } else {
        for i in 1..=9 {
            graph[k as usize].push((i as usize, i as i64));
        }
    }

    for i in 0..k {
        for x in 0..=9 {
            graph[i as usize].push((((10 * i + x) % k) as usize, x));
        }
    }

    let mut dijkstra = Dijkstra::new(&graph);
    let distance = dijkstra.search(k as usize);

    println!("{}", distance[0]);
}
