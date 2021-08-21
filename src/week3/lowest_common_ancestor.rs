// 236. 二叉树的最近公共祖先
// 给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。
//
// 百度百科中最近公共祖先的定义为："对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，
// 满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。"
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree

use std::rc::Rc;
use std::cell::RefCell;
use super::{Solution, TreeNode};

// 思路，直接递归查找左子树和有子树是否包含p,q节点，有以下几种可能：
// 1. 左子树和右子树均不为空。这种情况说明必然一左一右分别包含p,q，符合最近公共祖先定义，直接返回root
// 2. 左子树和右子树均为空。这种情况说明当前节点和左右子树没有任何关系，直接返回None
// 3. 左右子树只有一个为空。说明：
//     - 要么已经找到最近公共祖先，只需返回非空的最近公共祖先即可；
//     - 要么只找到一个节点，也只需返回这个找到的节点即可，供后续查找。
// 时间复杂度：O(n), 需要遍历所有节点
// 空间复杂度：O(n), 无需额外存储，存在递归调用
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>,
                                  p: Option<Rc<RefCell<TreeNode>>>,
                                  q: Option<Rc<RefCell<TreeNode>>>)
                                  -> Option<Rc<RefCell<TreeNode>>> {

        // 遍历到了叶子节点活着p,q节点，直接返回节点（nil）活着当前节点即可
        if root.is_none() || root == p || root == q {
            return root;
        }

        let left_val = match root.as_ref().unwrap().borrow().left.as_ref() {
            Some(node) => Solution::lowest_common_ancestor(
                Some(Rc::clone(node)),
                Some(Rc::clone(p.as_ref().unwrap())),
                Some(Rc::clone(q.as_ref().unwrap()))),
            None => None
        };

        let right_val = match root.as_ref().unwrap().borrow().right.as_ref() {
            Some(node) => Solution::lowest_common_ancestor(
                Some(Rc::clone(node)),
                Some(Rc::clone(p.as_ref().unwrap())),
                Some(Rc::clone(q.as_ref().unwrap()))),
            None => None
        };

        if left_val.is_some() && right_val.is_some() {
            return root;
        }
        if left_val.is_none() {
            return right_val;
        }
        if right_val.is_none() {
            return left_val;
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node0 = Rc::new(RefCell::new(TreeNode::new(0)));
        let node8 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        node3.borrow_mut().left = Some(Rc::clone(&node5));
        node3.borrow_mut().right = Some(Rc::clone(&node1));
        node5.borrow_mut().left = Some(Rc::clone(&node6));
        node5.borrow_mut().right = Some(Rc::clone(&node2));
        node1.borrow_mut().left = Some(Rc::clone(&node0));
        node1.borrow_mut().right = Some(Rc::clone(&node8));
        node2.borrow_mut().left = Some(Rc::clone(&node7));
        node2.borrow_mut().right = Some(Rc::clone(&node4));

        assert_eq!(Some(Rc::clone(&node3)),
                   Solution::lowest_common_ancestor(
                       Some(Rc::clone(&node3)),
                       Some(Rc::clone(&node5)),
                       Some(Rc::clone(&node1)),
                   ));
    }
}