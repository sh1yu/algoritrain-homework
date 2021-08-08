//283. 移动零
//
//给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 示例:
//
// 输入: [0,1,0,3,12]
// 输出: [1,3,12,0,0]
// 说明:
//
// 必须在原数组上操作，不能拷贝额外的数组。
// 尽量减少操作次数。
use super::{Solution};

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                if i != j {
                    nums[j] = nums[i];
                    nums[i] = 0;
                }
                j += 1;
            }
            // if i == j && nums[j] != 0 {
            //     j += 1;
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec1);
        println!("{:?}", vec1);

        let mut vec2 = vec![];
        Solution::move_zeroes(&mut vec2);
        println!("{:?}", vec2);

        let mut vec3 = vec![0, 0, 0];
        Solution::move_zeroes(&mut vec3);
        println!("{:?}", vec3);
    }
}