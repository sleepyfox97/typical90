## 解答方針
木の直径 + 1を求めることで解が得られる．<br>
実装としては、dfsを2回行って木の直径を出す．<br>

## 実装メモ
### dfsについて
DFS(Depth First Search)のデモコードとしては、以下のコードが考えられる．基本的にはこのコードを発展させていけば良い．<br>
DFSはstackを用いてFist In Last Outでの探索を行うことで実装できる．<br>
[play ground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7377d645782077ca69a5241331927f79)<br>
```rust
fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut seen = vec![false; graph.len()];
    let mut order = Vec::new();
    
    stack.push(start);

    while let Some(v) = stack.pop() {
        if !seen[v] {
            seen[v] = true;
            order.push(v);
        
            for &next in graph[v].iter().rev() { //ここで逆順にしないとstackにpushするところで若い順で処理されない．深さ優先探索においては、別に逆順にしなくても探索自体は正しく行われる．
                if !seen[next] {
                    stack.push(next);
                }
            }
        }
    }

    order
}

fn main() {
    let graph = vec![
        vec![1, 2],     // 0
        vec![0, 3, 4],  // 1
        vec![0],        // 2
        vec![1],        // 3
        vec![1],        // 4
    ];

    let order = dfs(&graph, 0);

    println!("Visit order: {:?}", order);
}
```

### Someについて
まず、RustのOption型について理解する必要がある．[Option](https://doc.rust-jp.rs/rust-by-example-ja/std/option.html)<br>

`Option<T>`は取得できないかもしれない値を表現する列挙型.[参照](https://qiita.com/take4s5i/items/c890fa66db3f71f41ce7)<br>

`Option<T>`は、値が存在しないことを意味する`None`と、値が存在することを示す`Some(value)`の二つの値をもつ．<br>

以下のように書くことで、「Someの時のみ、その値を受け取り処理する」といったコードが書ける
```rust
fn main() {
    let some_value: Option<usize> = Some(1);
    let none_value: Option<usize> = None;

    //処理される
    if let Some(v) = some_value {
        println!("some value = {}", v);
    } else {
        println!("none value");
    }
    
    //処理されない
    if let Some(v) = none_value {
        println!("some value = {}", v);
    } else {
        println!("none value");
    }
}
```