// 105. 从前序与中序遍历序列构造二叉树
// 给定一棵树的前序遍历 preorder 与中序遍历  inorder。请构造二叉树并返回其根节点。
//
// https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

use std::rc::Rc;
use std::cell::RefCell;
use super::{Solution, TreeNode};

// 思路：使用前序遍历的第一个节点对中序遍历进行二分，中序遍历左侧为其左子树，右侧为其右子树
// 递归建立左右子树的根节点，然后挂在当前节点上即可
// 时间复杂度： O(n^2), 每一个节点都需要处理一次，每次处理需要n的查找时间
// 空间复杂度： O(n)
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        }

        let root_val = preorder[0];
        let index = Solution::search(root_val, &inorder).unwrap();

        let mut root = TreeNode::new(root_val);
        if index > 0 {
            root.left = Solution::build_tree(preorder[1..index + 1].to_vec(), inorder[0..index].to_vec());
        }
        if index < preorder.len() {
            root.right = Solution::build_tree(preorder[index + 1..].to_vec(), inorder[index + 1..].to_vec());
        }
        Some(Rc::new(RefCell::new(root)))
    }

    // 暴力搜索
    fn search(val: i32, order: &Vec::<i32>) -> Result<usize, usize> {
        for (i, &num) in order.iter().enumerate() {
            if num == val {
                return Ok(i);
            }
        }
        Err(usize::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node9 = Rc::new(RefCell::new(TreeNode::new(9)));
        let node20 = Rc::new(RefCell::new(TreeNode::new(20)));
        let node15 = Rc::new(RefCell::new(TreeNode::new(15)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        node3.borrow_mut().left = Some(Rc::clone(&node9));
        node3.borrow_mut().right = Some(Rc::clone(&node20));
        node20.borrow_mut().left = Some(Rc::clone(&node15));
        node20.borrow_mut().right = Some(Rc::clone(&node7));

        assert_eq!(Some(Rc::clone(&node3)),
                   Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]));
    }
}