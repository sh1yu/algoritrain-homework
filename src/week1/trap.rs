//42. 接雨水
// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

use super::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut ans = 0;

        for i in 0..height.len() {
            while stack.len() > 0 && height[i] > height[stack[stack.len() - 1]] {
                let t = stack.pop().unwrap();
                if stack.len() == 0 {
                    break;
                }
                let l = stack[stack.len() - 1];
                let w = i - l - 1;
                let h = std::cmp::min(height[i], height[l]) - height[t];
                ans += (w as i32) * h;
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        println!("{:?}", Solution::trap(vec![4, 2, 0, 3, 2, 5]));
    }
}