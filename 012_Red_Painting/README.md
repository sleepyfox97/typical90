## 解答方針
union-find木を用いて連携判定を行う問題<br>
## 実装メモ
### union-find
union find木の実装
```rust
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
  
  fn union(&mut self, x: usize,y: usize) {
    let root_x = self.find(x);
    let root_y = self.find(y);
    
    if root_x != root_y {
      if self.rank[root_x] > self.rank[root_y] {
        self.parent[root_y] = root_x;
      } else if self.rank[root_x] < self.rank[root_y] {
        self.parent[root_x] = root_y;
      } else {
        self.parent[root_y] = root_x;
        self.rank[root_x] += 1;
      }
    }
  }
```