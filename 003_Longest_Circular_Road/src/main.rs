use proconio::input;

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n - 1],
    }

    let mut graph = vec![Vec::new(); n+1];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    // 任意の頂点 (0) から最も遠い頂点を見つける
    let (farthest_node, _) = dfs(&graph, 1);

    // その頂点から最も遠い頂点との距離を求める
    let (_, diameter) = dfs(&graph, farthest_node);

    println!("{}", diameter + 1);
}


fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> (usize, usize) {
    let mut stack = Vec::new();
    let mut distances = vec![None; graph.len()];
    distances[start] = Some(0);
    stack.push((start, 0));

    let mut farthest_node = start;
    let mut max_distance = 0;

    while let Some((v, dist)) = stack.pop() {
        if dist > max_distance {
            max_distance = dist;
            farthest_node = v;
        }

        for &next in &graph[v] {
            if distances[next].is_none() {
                distances[next] = Some(dist + 1);
                stack.push((next, dist + 1));
            }
        }
    }

    (farthest_node, max_distance)
}

// 木の深さを適切に求められているかのテスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple_tree() {
        let graph = vec![
            vec![1, 2],     // Node 0 is connected to Node 1 and Node 2
            vec![0, 3, 4],  // Node 1 is connected to Node 0, Node 3 and Node 4
            vec![0],        // Node 2 is connected to Node 0
            vec![1],        // Node 3 is connected to Node 1
            vec![1],        // Node 4 is connected to Node 1
        ];

        let (farthest_node, max_distance) = dfs(&graph, 0);
        assert_eq!(farthest_node, 4); // or 3
        assert_eq!(max_distance, 2);
    }

    #[test]
    fn test_dfs_single_node() {
        let graph = vec![
            vec![],  // Single node with no edges
        ];

        let (farthest_node, max_distance) = dfs(&graph, 0);
        assert_eq!(farthest_node, 0);
        assert_eq!(max_distance, 0);
    }

    #[test]
    fn test_dfs_linear_tree() {
        let graph = vec![
            vec![1],     // Node 0 is connected to Node 1
            vec![0, 2],  // Node 1 is connected to Node 0 and Node 2
            vec![1, 3],  // Node 2 is connected to Node 1 and Node 3
            vec![2],     // Node 3 is connected to Node 2
        ];

        let (farthest_node, max_distance) = dfs(&graph, 0);
        assert_eq!(farthest_node, 3);
        assert_eq!(max_distance, 3);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let graph = vec![
            vec![1],     // Component 1: Node 0 is connected to Node 1
            vec![0],     // Component 1: Node 1 is connected to Node 0
            vec![3],     // Component 2: Node 2 is connected to Node 3
            vec![2],     // Component 2: Node 3 is connected to Node 2
        ];

        let (farthest_node, max_distance) = dfs(&graph, 0);
        assert_eq!(farthest_node, 1);
        assert_eq!(max_distance, 1);
    }
}