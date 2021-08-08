use std::fmt::{Debug};
use std::rc::Rc;
use std::cell::RefCell;

mod move_zeros;
mod merge_two_lists;
mod circular_deque;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

// Definition for tree-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RcListNode {
    val: i32,
    next: Option<Rc<RefCell<RcListNode>>>,
}

impl RcListNode {
    pub fn new(val: i32) -> RcListNode {
        RcListNode {
            val: val,
            next: None,
        }
    }
}

// Solution
pub struct Solution {}

impl Solution {
    pub fn format_list(head: &Option<Box<ListNode>>) -> String {
        return match head {
            None => String::from("None"),
            Some(n) => format!("{} -> {}", n.val, Solution::format_list(&n.next))
        };
    }
}
