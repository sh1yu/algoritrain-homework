// 1. 两数之和
// 给定一个整数数组nums和一个整数目标值target，请你在该数组中找出和为目标值target的那两个整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum

use super::Solution;
use std::collections::HashMap;

// 思路：使用hash记录每个数字的出现，以O(1)的速度查找目标
// 时间复杂度：O(n)
// 空间复杂度：O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {

            // map中存在，直接返回map中的位置和当前位置
            if let Some(&n) = map.get(&num) {
                return vec![n as i32, i as i32];
            }

            // 不存在，map写入，往后查找
            map.insert(target - num, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?} -- expect [0, 1]", Solution::two_sum(vec![2, 7, 11, 15], 9));
        println!("{:?} -- expect [1, 2]", Solution::two_sum(vec![3, 2, 4], 6));
        println!("{:?} -- expect [0, 1]", Solution::two_sum(vec![3, 3], 6));
    }
}