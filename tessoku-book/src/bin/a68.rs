use proconio::input;

#[derive(Clone, Debug, Copy)]
#[allow(dead_code)]
struct Edge {
    pub from: usize,
    pub to: usize,
    pub capacity: i64,
    pub rev: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Graph {
    pub n: usize,
    pub m: usize,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize, edges: &Vec<(usize, usize, i64)>, zero_indexed: bool) -> Graph {
        let mut graph = vec![Vec::new(); n];
        match zero_indexed {
            true => {
                for (from, to, capacity) in edges.iter() {
                    let rev_position_from = graph[*to].len();
                    let rev_position_to = graph[*from].len();

                    graph[*from].push(Edge {
                        from: *from,
                        to: *to,
                        capacity: *capacity,
                        rev: rev_position_from,
                    });

                    graph[*to].push(Edge {
                        from: *to,
                        to: *from,
                        capacity: 0,
                        rev: rev_position_to,
                    });
                }
            }
            false => {
                for (from, to, capacity) in edges.iter() {
                    let from = *from - 1;
                    let to = *to - 1;
                    let rev_position_from = graph[to].len();
                    let rev_position_to = graph[from].len();

                    graph[from].push(Edge {
                        from: from,
                        to: to,
                        capacity: *capacity,
                        rev: rev_position_from,
                    });

                    graph[to].push(Edge {
                        from: to,
                        to: from,
                        capacity: 0,
                        rev: rev_position_to,
                    });
                }
            }
        };

        return Graph {
            n: n,
            m: edges.len(),
            edges: graph,
        };
    }

    fn dfs(
        &mut self,
        explored: &mut Vec<bool>,
        current: usize,
        destination: usize,
        minimum_flow: i64,
    ) -> i64 {
        if current == destination {
            return minimum_flow;
        }

        for i in 0..self.edges[current].len() {
            let edge = self.edges[current][i];

            if explored[edge.to] || edge.capacity == 0 {
                continue;
            }

            explored[edge.to] = true;

            let flow = self.dfs(
                explored,
                edge.to,
                destination,
                std::cmp::min(minimum_flow, edge.capacity),
            );

            if flow > 0 {
                self.edges[current][i].capacity -= flow;
                self.edges[edge.to][edge.rev].capacity += flow;
                return flow;
            }
        }

        return 0;
    }

    fn get_minimum_flow(&mut self, start: usize, destination: usize) -> i64 {
        let mut minimum_flow: i64 = 0;
        loop {
            let mut explored = vec![false; self.n];

            let flow = self.dfs(&mut explored, start, destination, 1i64 << 60);
            if flow > 0 {
                minimum_flow += flow;
            } else {
                break;
            }
        }

        return minimum_flow;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m]
    };

    let mut graph = Graph::new(n, &abc, false);

    println!("{}", graph.get_minimum_flow(0, n - 1));
}
