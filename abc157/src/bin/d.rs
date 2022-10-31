use proconio::input;
use std::mem;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

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
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); k]
    };

    let mut f: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut b: Vec<Vec<usize>> = vec![Vec::new(); n];

    let mut uf = UnionFind::new(n);
    for &(a, b) in ab.iter() {
        f[a - 1].push(b - 1);
        f[b - 1].push(a - 1);
        uf.unite(a - 1, b - 1);
    }

    for &(c, d) in cd.iter() {
        b[c - 1].push(d - 1);
        b[d - 1].push(c - 1);
    }

    let mut answer = vec![0; n];
    for i in 0..n {
        answer[i] = uf.get_size(i) - 1;
        answer[i] -= f[i].len();
        for &j in b[i].iter() {
            if uf.is_same(i, j) {
                answer[i] -= 1;
            }
        }
    }

    println!(
        "{}",
        answer
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
