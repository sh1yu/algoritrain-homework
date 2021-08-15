// 剑指 Offer 49. 丑数
// 我们把只包含质因子 2、3 和 5 的数称作丑数（Ugly Number）。求按从小到大的顺序的第 n 个丑数。
//
// https://leetcode-cn.com/problems/chou-shu-lcof/

use super::Solution;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

// 思路：直接堆排序+hash过滤
// 时间复杂度：O(n)
// 空间复杂度：O(n)
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut set = HashSet::new();
        let mut heap = BinaryHeap::new();
        set.insert(1);
        heap.push(Reverse(1));
        let mut curr = 0;
        for _ in 0..n {
            curr = heap.pop().unwrap().0;
            let mut next = curr as i64 * 2;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
            next = curr as i64 * 3;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
            next = curr as i64 * 5;
            if next < i32::MAX as i64 && set.insert(next) {
                heap.push(Reverse(next as i32));
            }
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::nth_ugly_number(1));
        println!("{:?}", Solution::nth_ugly_number(10));
        println!("{:?}", Solution::nth_ugly_number(1407));
    }
}
