//26. 删除有序数组中的重复项
//给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。
//
// 不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
//
use super::{Solution};

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut slow = 0;
        for fast in 1..nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        (slow + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![0, 0, 0, 1, 3, 3, 12];
        println!("{} {:?}", Solution::remove_duplicates(&mut vec1), vec1);

        let mut vec2 = vec![];
        println!("{} {:?}", Solution::remove_duplicates(&mut vec2), vec2);

        let mut vec3 = vec![0, 0, 0];
        println!("{} {:?}", Solution::remove_duplicates(&mut vec3), vec3);
    }
}