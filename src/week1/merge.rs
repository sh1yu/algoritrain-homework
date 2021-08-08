//88. 合并两个有序数组
//给你两个有序整数数组nums1 和 nums2，请你将 nums2 合并到nums1中，使 nums1 成为一个有序数组。
//
// 初始化nums1 和 nums2 的元素数量分别为m 和 n 。
// 你可以假设nums1 的空间大小等于m + n，这样它就有足够的空间保存来自 nums2 的元素。
//
use super::{Solution};

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m + n - 1;
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        while i >= 0 {
            if p1 >= 0 && p2 >= 0 {
                if nums1[p1 as usize] >= nums2[p2 as usize] {
                    nums1[i as usize] = nums1[p1 as usize];
                    p1 -= 1;
                } else {
                    nums1[i as usize] = nums2[p2 as usize];
                    p2 -= 1;
                }
            } else if p1 >= 0 {
                nums1[i as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else if p2 >= 0 {
                nums1[i as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![10, 20, 30, 40, 50, 60, 70, 0, 0, 0];
        let mut vec2 = vec![32, 34, 39];
        Solution::merge(&mut vec1, 7, &mut vec2, 3);
        println!("{:?}", vec1);
    }
}