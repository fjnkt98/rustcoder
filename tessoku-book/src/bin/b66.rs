use std::collections::HashSet;
use std::mem;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let parent = vec![std::usize::MAX; n];
        let rank = vec![0; n];
        let size = vec![1; n];

        return UnionFind { parent, rank, size };
    }

    pub fn get_root(&mut self, x: usize) -> usize {
        if self.parent[x] == std::usize::MAX {
            return x;
        } else {
            self.parent[x] = self.get_root(self.parent[x]);
            return self.parent[x];
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        return self.get_root(x) == self.get_root(y);
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.get_root(x);
        let mut ry = self.get_root(y);

        if rx == ry {
            return false;
        }

        if self.rank[rx] < self.rank[ry] {
            mem::swap(&mut rx, &mut ry);
        }

        self.parent[ry] = rx;

        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }

        self.size[rx] += self.size[ry];

        return true;
    }

    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.get_root(x);
        return self.size[root];
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let tmp = line
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect::<Vec<usize>>();
    let (n, m) = (tmp[0], tmp[1]);

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        let tmp = line
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect::<Vec<usize>>();
        edges.push((tmp[0] - 1, tmp[1] - 1));
    }

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let q: usize = line.trim().parse().ok().unwrap();

    let mut query: Vec<Vec<i64>> = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        let tmp = line
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect::<Vec<i64>>();
        query.push(tmp.clone());
    }

    let mut uf = UnionFind::new(n);
    let mut exclude = HashSet::new();
    for query in query.iter().filter(|q| q[0] == 1) {
        exclude.insert(query[1] as usize - 1);
    }
    for (i, &(a, b)) in edges.iter().enumerate() {
        if !exclude.contains(&i) {
            uf.unite(a, b);
        }
    }

    let mut answer = Vec::new();
    for query in query.iter().rev() {
        if query[0] == 1 {
            let x = query[1] as usize - 1;
            let (a, b) = edges[x];
            uf.unite(a, b);
        } else {
            let u = query[1] as usize - 1;
            let v = query[2] as usize - 1;

            if uf.is_same(u, v) {
                answer.push(String::from("Yes"));
            } else {
                answer.push(String::from("No"));
            }
        }
    }

    answer.reverse();
    println!("{}", answer.join("\n"));
}
