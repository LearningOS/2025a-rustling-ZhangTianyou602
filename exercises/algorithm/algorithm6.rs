use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表：adj[src] 存储 src 的所有邻接节点
}

impl Graph {
    // 创建包含 n 个节点的空图（节点索引从 0 开始）
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加无向边：src ↔ dest（双向添加邻接关系）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // DFS 辅助函数：递归执行深度优先搜索
    // v: 当前访问的节点索引
    // visited: 已访问节点的集合（可变引用，实时更新）
    // visit_order: 访问顺序列表（可变引用，记录结果）
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 1. 标记当前节点为已访问，并加入访问顺序
        visited.insert(v);
        visit_order.push(v);

        // 2. 遍历当前节点的所有邻接节点
        for &neighbor in &self.adj[v] {
            // 3. 若邻接节点未访问，递归访问
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
            // 若已访问（如环、自环、重复边），直接跳过
        }
    }

    // 对外暴露的 DFS 接口：从 start 节点开始遍历，返回访问顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // 记录已访问节点
        let mut visit_order = Vec::new(); // 记录访问顺序
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        // 测试简单线性图：0 - 1 - 2
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        // 测试带环图：0-1-2-0，2-3（含自环 3-3）
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); // 自环

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
        // 说明：自环 3-3 被跳过（已访问），环 0-1-2-0 被避免重复访问
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        // 测试非连通图：组件1（0-1-2），组件2（3-4）
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        // 从 0 开始，只访问组件1
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);

        // 从 3 开始，只访问组件2
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }

    #[test]
    fn test_dfs_single_node() {
        // 测试单节点图（无边）
        let graph = Graph::new(1);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0]);
    }

    #[test]
    fn test_dfs_self_loop() {
        // 测试单节点带自环
        let mut graph = Graph::new(1);
        graph.add_edge(0, 0); // 自环

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0]); // 自环被跳过，只访问一次
    }

    #[test]
    fn test_dfs_duplicate_edges() {
        // 测试重复边：0-1 重复添加两次
        let mut graph = Graph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(0, 1); // 重复边

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1]); // 重复边不影响访问顺序，只访问一次
    }
}