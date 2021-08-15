// 94. 二叉树的中序遍历
// 给定一个二叉树的根节点 root ，返回它的 中序 遍历。
// https://leetcode-cn.com/problems/binary-tree-inorder-traversal/

use super::{Solution, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    // 思路：递归
    // 时间复杂度： O(n)
    // 空间复杂度： O(n)
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(node) = root {
            ans.append(&mut Solution::inorder_traversal(node.borrow_mut().left.take()));
            ans.push(node.borrow().val);
            ans.append(&mut Solution::inorder_traversal(node.borrow_mut().right.take()));
        }
        ans
    }

    // 思路：迭代
    // 时间复杂度： O(n)
    // 空间复杂度： O(n)
    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        if root.is_none() {
            return ans;
        }
        stack.push(root.unwrap());

        while !stack.is_empty() {
            let mut curr = stack.pop();
            while curr.is_some() {
                // 一直添加左子树直到为空
                let node = curr.unwrap();
                curr = node.borrow_mut().left.take();
                stack.push(node);
            }

            curr = stack.pop();
            ans.push(curr.as_mut().unwrap().borrow().val);
            let right = curr.unwrap().borrow_mut().right.take();
            if right.is_some() {
                stack.push(right.unwrap());
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        node2.borrow_mut().left = Some(node3);
        node1.borrow_mut().right = Some(node2);
        println!("{:?}", Solution::inorder_traversal(Some(node1)));
    }

    #[test]
    fn it_works2() {
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        node2.borrow_mut().left = Some(node3);
        node1.borrow_mut().right = Some(node2);
        println!("{:?}", Solution::inorder_traversal2(Some(node1)));
    }
}