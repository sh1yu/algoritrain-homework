//42. 接雨水
// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

use super::Solution;

// 思路：使用栈
// 之前自己想到的一个思路也是用栈，但是问题在于写入栈的内容有问题：将所有数据进行入栈，然后在U型槽后进行计算容量。
// 问题在于无法有效地判断何时计算容量，可能一个小的U型槽是大的U型槽的一部分，所以容量计算是偏小的。
// 这里将U型槽左侧边界位置入栈（栈中数据指向的位置单调递减），每次计算U型槽"超出"的矩形的容量而不是"真正"的容量，
// 把所有容量全部加上了，所以正确。
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