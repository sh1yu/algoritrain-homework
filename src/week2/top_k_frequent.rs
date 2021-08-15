// 347. 前 K 个高频元素
// 给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。你可以按 任意顺序 返回答案。
// https://leetcode-cn.com/problems/top-k-frequent-elements/

use super::Solution;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Eq)]
struct MyTuple {
    item: (i32, i32),
}

impl MyTuple {
    pub fn new(item: (i32, i32)) -> MyTuple {
        MyTuple {
            item
        }
    }
}

impl PartialEq for MyTuple {
    fn eq(&self, other: &Self) -> bool {
        self.item.1 == other.item.1
    }
}

impl PartialOrd for MyTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other)) // Delegate to the implementation in `Ord`.
    }
}

impl Ord for MyTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.item.1).cmp(&Reverse(other.item.1))
    }
}


// 思路：直接用堆记录top k个值
// 时间复杂度：O(nk)
// 空间复杂度: O(n)
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut heap: BinaryHeap<MyTuple> = BinaryHeap::with_capacity(k as usize);
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for (num, count) in map {
            // 如果堆中数量未达到k,直接入堆；否则出一个入一个
            if heap.len() < k as usize {
                heap.push(MyTuple::new((num, count)));
            } else if heap.peek().unwrap().item.1 < count {
                heap.pop();
                heap.push(MyTuple::new((num, count)));
            }
        }

        heap.iter().map(|x| x.item.0).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
        println!("{:?}", Solution::top_k_frequent(vec![1], 1));
        println!("{:?}", Solution::top_k_frequent(vec![1, 2], 2));
    }
}