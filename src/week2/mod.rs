mod is_anagram;
mod two_sum;
mod group_anagrams;
mod inorder_traversal;
mod nth_ugly_number;
mod top_k_frequent;

use std::fmt::{Debug};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Solution
pub struct Solution {}
