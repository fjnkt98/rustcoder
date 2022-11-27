use proconio::input;
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
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, i64); m]
    };

    abc.sort_by_key(|e| std::cmp::Reverse(e.2));

    let mut uf = UnionFind::new(n);
    let mut answer = 0;
    for &(a, b, c) in abc.iter() {
        let a = a - 1;
        let b = b - 1;

        if !uf.is_same(a, b) {
            uf.unite(a, b);
            answer += c;
        }
    }
    println!("{}", answer);
}
