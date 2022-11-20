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
struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize, edges: &Vec<Vec<Edge>>) -> Graph {
        return Graph {
            n,
            edges: edges.clone(),
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
        c: [String;n]
    };

    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); 2 * n + 2];
    for (i, s) in c.iter().enumerate() {
        for (j, &c) in s.chars().collect::<Vec<char>>().iter().enumerate() {
            let from = i;
            let to = j + n;
            if c == '#' {
                let rev_position_from = graph[to].len();
                let rev_position_to = graph[from].len();
                graph[from].push(Edge {
                    from,
                    to,
                    capacity: 1,
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
    }

    for to in 0..n {
        let from = 2 * n;
        let rev_position_from = graph[to].len();
        let rev_position_to = graph[from].len();

        graph[from].push(Edge {
            from,
            to,
            capacity: 1,
            rev: rev_position_from,
        });

        graph[to].push(Edge {
            from: to,
            to: from,
            capacity: 0,
            rev: rev_position_to,
        });
    }

    for from in n..2 * n {
        let to = 2 * n + 1;
        let rev_position_from = graph[to].len();
        let rev_position_to = graph[from].len();

        graph[from].push(Edge {
            from,
            to,
            capacity: 1,
            rev: rev_position_from,
        });

        graph[to].push(Edge {
            from: to,
            to: from,
            capacity: 0,
            rev: rev_position_to,
        });
    }

    let mut graph = Graph::new(2 * n + 2, &graph);

    println!("{}", graph.get_minimum_flow(2 * n, 2 * n + 1));
}
