//189. 旋转数组
//给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
//
use super::{Solution};


impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let mod_k = k as usize % len;
        nums.reverse();
        nums[0..mod_k].reverse();
        nums[mod_k..len].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut vec1, 2);
        println!("{:?}", vec1);
    }
}