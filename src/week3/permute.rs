// 46. 全排列
// 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

use super::Solution;

// 思路：直接对每个位置上的数字和其他位置进行替换，参考官方解答。
// 使用first指针对数组进行了分割，前面的是已经排好序的，后面的是未排好序的
// 时间复杂度 O(n * n!)
// 空间复杂度 O(n)
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let size = nums.len();
        let mut output = nums;
        Solution::backtrack_permute(&mut res, &mut output, 0, size);
        res
    }

    fn backtrack_permute(res: &mut Vec<Vec<i32>>, output: &mut Vec<i32>, first: usize, len: usize) {
        if first == len {
            res.push(output.to_vec());
            return;
        }

        for i in first..len {
            // 动态维护数组
            output.swap(i, first);
            // 继续递归填下一个数
            Solution::backtrack_permute(res, output, first + 1, len);
            // 撤销操作
            output.swap(i, first);
        }
    }

    pub fn permute2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut nums: Vec<i32> = nums;
        let mut visit = vec![false; nums.len()];
        let mut path: Vec<i32> = Vec::new();
        Solution::backtrack_permute2(&mut res, &mut nums, 0, &mut visit, &mut path);
        res
    }

    // 通用的backtrack，记录path与visit
    fn backtrack_permute2(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, depth: usize, visit: &mut Vec<bool>, path: &mut Vec<i32>) {
        if depth == nums.len() {
            res.push(path.to_vec());
            return;
        }

        for i in 0..nums.len() {
            if visit[i] {
                continue;
            }

            // 动态维护数组
            path.push(nums[i]);
            visit[i] = true;
            // 继续递归填下一个数
            Solution::backtrack_permute2(res, nums, depth + 1, visit, path);
            // 撤销操作
            path.pop();
            visit[i] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut expect = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        expect.sort();

        let mut actual = Solution::permute([1, 2, 3].to_vec());
        actual.sort();
        assert_eq!(expect, actual);

        let mut actual2 = Solution::permute2([1, 2, 3].to_vec());
        actual2.sort();
        assert_eq!(expect, actual2);
    }
}