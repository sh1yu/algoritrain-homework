//21. 合并两个有序链表
//将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

use super::{ListNode, Solution};

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut tail = &mut result;

        let mut remain1 = l1;
        let mut remain2 = l2;

        while remain1.is_some() || remain2.is_some() {
            if remain1.is_some() && remain2.is_some() {
                let val1 = remain1.as_ref().unwrap().as_ref().val;
                let val2 = remain2.as_ref().unwrap().as_ref().val;
                if val1 < val2 {
                    let mut n1 = remain1.unwrap();
                    remain1 = n1.next.take();
                    tail.next = Option::Some(n1);
                    tail = tail.next.as_mut().unwrap().as_mut();
                } else if val1 > val2 {
                    let mut n2 = remain2.unwrap();
                    remain2 = n2.next.take();
                    tail.next = Option::Some(n2);
                    tail = tail.next.as_mut().unwrap().as_mut();
                } else {
                    let mut n1 = remain1.unwrap();
                    remain1 = n1.next.take();
                    let mut n2 = remain2.unwrap();
                    remain2 = n2.next.take();
                    n1.next = Option::Some(n2);
                    tail.next = Option::Some(n1);
                    tail = tail.next.as_mut().unwrap().next.as_mut().unwrap().as_mut();
                }
            } else if remain1.is_some() {
                tail.next = remain1.take();
                break;
            } else if remain2.is_some() {
                tail.next = remain2.take();
                break;
            }
        }

        result.next
    }

    pub fn merge_two_lists2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) =>
                if n1.val < n2.val {
                    Some(Box::new(ListNode { val: n1.val, next: Solution::merge_two_lists(n1.next, Some(n2)) }))
                } else {
                    Some(Box::new(ListNode { val: n2.val, next: Solution::merge_two_lists(Some(n1), n2.next) }))
                }
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut l1 = Option::Some(Box::new(ListNode::new(10)));
        l1.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(20)));
        l1.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(30)));

        let mut l2 = Option::Some(Box::new(ListNode::new(5)));
        l2.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(12)));
        l2.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(18)));

        // println!("{:?}", Solution::merge_two_lists(l1, l2));
        println!("{}", Solution::format_list(&Solution::merge_two_lists2(l1, l2)));
    }
}