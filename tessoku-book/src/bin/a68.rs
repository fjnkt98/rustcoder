use proconio::input;

#[derive(Clone, Debug, Copy)]
struct Edge {
    pub capacity: i64,
    pub to: usize,
    pub rev: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    v: Vec<Vec<Edge>>,
    flow: i64,
}

impl Graph {
    fn new(n: usize) -> Graph {
        let v = vec![Vec::new(); n];
        return Graph { v, flow: 0 };
    }

    fn add(&mut self, a: usize, b: usize, c: i64) {
        let rev_position_a = self.v[b].len();
        let rev_position_b = self.v[a].len();

        self.v[a].push(Edge {
            capacity: c,
            to: b,
            rev: rev_position_a,
        });

        self.v[b].push(Edge {
            capacity: 0,
            to: a,
            rev: rev_position_b,
        });
    }

    fn find_path(&mut self, start: usize, end: usize) -> i64 {
        let mut explored = vec![false; self.v.len()];
        explored[start] = true;

        return self.find_path_recursive(&mut explored, start, end, 1i64 << 60);
    }

    fn find_path_recursive(
        &mut self,
        explored: &mut Vec<bool>,
        current: usize,
        destination: usize,
        minimum_flow: i64,
    ) -> i64 {
        if current == destination {
            return minimum_flow;
        }

        for i in 0..self.v[current].len() {
            let edge = self.v[current][i];

            if explored[edge.to] || edge.capacity == 0 {
                continue;
            }

            explored[edge.to] = true;

            let flow = self.find_path_recursive(
                explored,
                edge.to,
                destination,
                std::cmp::min(minimum_flow, edge.capacity),
            );

            if flow > 0 {
                self.v[current][i].capacity -= flow;
                self.v[edge.to][edge.rev].capacity += flow;
                return flow;
            }
        }

        return 0;
    }

    fn add_flow(&mut self, start: usize, destination: usize) -> i64 {
        let flow = self.find_path(start, destination);
        self.flow += flow;
        return flow;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m]
    };

    let mut graph = Graph::new(n);

    for &(a, b, c) in abc.iter() {
        let a = a - 1;
        let b = b - 1;
        graph.add(a, b, c);
    }

    loop {
        let flow = graph.add_flow(0, n - 1);
        if flow == 0 {
            break;
        }
    }

    println!("{}", graph.flow);
}
