package week2;

//589. N 叉树的前序遍历
//给定一个 N 叉树，返回其节点值的 前序遍历 。
//
//N 叉树 在输入中按层序遍历进行序列化表示，每组子节点由空值 null 分隔（请参见示例）。
// https://leetcode-cn.com/problems/n-ary-tree-preorder-traversal/
//
// rust没有合适的数据结构，此问题使用java解答


import java.util.*;

class Node {
    public int val;
    public List<Node> children;

    public Node() {
    }

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
}

public class PreOrder {

    // 思路：迭代
    // 时间复杂度：O(n)
    // 空间复杂度：O(n)
    public List<Integer> preorder(Node root) {
        if (root == null) {
            return  new LinkedList<>();
        }
        Deque<Node> stack = new ArrayDeque<>();
        stack.push(root);
        List<Integer> res = new LinkedList<>();

        while (!stack.isEmpty()) {
            Node curr = stack.pop();
            res.add(curr.val);
            if (curr.children != null) {
                for (int i = curr.children.size() - 1; i >= 0; i--) {
                    stack.push(curr.children.get(i));
                }
            }
        }

        return res;
    }
}
