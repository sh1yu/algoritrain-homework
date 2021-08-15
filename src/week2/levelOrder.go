//429. N 叉树的层序遍历
//给定一个 N 叉树，返回其节点值的层序遍历。（即从左到右，逐层遍历）。
//
//树的序列化输入是用层序遍历，每组子节点都由 null 值分隔（参见示例）。
//
// rust没有合适的数据结构，此问题使用go解答
// https://leetcode-cn.com/problems/n-ary-tree-level-order-traversal/

// 思路：使用队列记录待处理节点即可
// 时间复杂度：O(n)
// 空间复杂度：O(n)
func levelOrder(root *Node) [][]int {
    res := make([][]int, 0)
    if root == nil {
        return res
    }

    // 当前层待处理节点
    current := []*Node{root}

    for len(current) > 0 {
        // 下一层待处理节点
        next := make([]*Node, 0)
        // 当前层遍历结果
        tmpRes := make([]int, 0)
        for _, node := range current {
            tmpRes = append(tmpRes, node.Val)
            for _, nextNode := range node.Children {
                next = append(next, nextNode)
            }
        }
        res = append(res, tmpRes)
        current = next
    }

    return res
}