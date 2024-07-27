use proconio::input;

struct UnionFind {
  parent: Vec<usize>,
  rank: Vec<usize>,
}

impl UnionFind {
  fn new(size: usize) -> Self {
    UnionFind {
      parent: (0..size).collect(),
      rank: vec![0; size],
    }
  }
  
  fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
      self.parent[x] = self.find(self.parent[x]);
    }
    self.parent[x]
  }
  
  fn union(&mut self, x: usize, y: usize) {
    let root_x = self.find(x);
    let root_y = self.find(y);
    
    if root_x != root_y {
      if self.rank[root_x] > self.rank[root_y] {
        self.parent[root_y] = root_x;
      } else if self.rank[root_y] > self.rank[root_x] {
        self.parent[root_x] = root_y;
      } else {
        self.parent[root_y] = root_x;
        self.rank[root_x] += 1;
      }
    }
  }
  
  fn is_connected(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }
}

fn main() {
  input!{
    h: usize,
    w: usize,
    q: usize
  };

  let mut v = vec![vec![0; w + 2]; h + 2];
  let mut uf = UnionFind::new(h * w);

  for _ in 0..q {
    input!{
      t: usize,
    };
    if t == 1 {
      input!{
        r: usize,
        c: usize,
      };
      v[r][c] = 1;
      let index = (r - 1) * w + (c - 1);
      if v[r - 1][c] == 1 {
        uf.union(index, (r - 2) * w + (c - 1));
      }
      if v[r + 1][c] == 1 {
        uf.union(index, r * w + (c - 1));
      }
      if v[r][c + 1] == 1 {
        uf.union(index, (r - 1) * w + c);
      }
      if v[r][c - 1] == 1 {
        uf.union(index, (r - 1) * w + (c - 2));
      }
    } else {
      input!{
        ra: usize,
        ca: usize,
        rb: usize,
        cb: usize,
      };
      if v[ra][ca] == 1 && v[rb][cb] == 1 && uf.is_connected((ra - 1) * w + (ca - 1), (rb - 1) * w + (cb - 1)) {
        println!("Yes");
      } else {
        println!("No");
      }
    }
  }
}
