// 47. 全排列 II
// 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。
// https://leetcode-cn.com/problems/permutations-ii/

use super::Solution;

// 思路： 和全排列I不同的是，需要先排序，然后限制相同数字的访问顺序
// 时间复杂度：O(n * n!)
// 空间复杂度：O(n)
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let size = nums.len();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        let mut path = vec![];
        let mut visit = vec![false; size];

        Solution::backtrack_permute_unique(&mut res, &mut nums, 0, &mut visit, &mut path);
        res
    }

    fn backtrack_permute_unique(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, depth: usize, visit: &mut Vec<bool>, path: &mut Vec<i32>) {
        if depth == nums.len() {
            res.push(path.to_vec());
            return;
        }

        for i in 0..nums.len() {
            if visit[i] || (i > 0 && nums[i] == nums[i - 1] && !visit[i - 1]) {
                continue;
            }
            // 动态维护数组
            path.push(nums[i]);
            visit[i] = true;
            // 继续递归填下一个数
            Solution::backtrack_permute_unique(res, nums, depth + 1, visit, path);
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
        let mut actual = Solution::permute_unique([1, 2, 3].to_vec());
        actual.sort();
        assert_eq!(expect, actual);
    }

    #[test]
    fn it_works2() {
        let mut expect = vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ];
        expect.sort();
        let mut actual = Solution::permute_unique([1, 1, 2].to_vec());
        actual.sort();
        assert_eq!(expect, actual);
    }
}