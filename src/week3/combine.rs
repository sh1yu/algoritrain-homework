// 77. 组合
// 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
//
// 你可以按 任何顺序 返回答案。
// https://leetcode-cn.com/problems/combinations/

use std::collections::HashSet;

struct Solution {
    tmp: Vec<i32>,
    res: Vec<Vec<i32>>,
}

// 思路：参考官方题解，使用标准化的回溯模版
// 时间复杂度：O(2^k)
// 空间复杂度：O(n)
impl Solution {
    // 尝试将tmp和res放到局部函数中.
    pub fn combine2(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut tmp = vec![];
        let mut res = vec![];
        //dfs不能定义为闭包，闭包的递归有些问题
        fn dfs(cur: i32, n: i32, k: i32, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            // 剪枝
            if tmp.len() as i32 + (n - cur + 1) < k {
                return;
            }

            if tmp.len() == k as usize {
                res.push(tmp.clone());
                return;
            }

            if cur > n {
                return;
            }

            tmp.push(cur);
            dfs(cur + 1, n, k, tmp, res);
            tmp.pop();
            // dfs(cur + 1, n, k, tmp, res);
        }
        dfs(1, n, k, &mut tmp, &mut res);
        res
    }

    // 使用结构体中的tmp和res
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut s = Solution { tmp: vec![], res: vec![] };
        s.dfs(1, n, k);
        s.res
    }

    fn dfs(&mut self, cur: i32, n: i32, k: i32) {
        // 剪枝
        if self.tmp.len() as i32 + (n - cur + 1) < k {
            return;
        }

        if self.tmp.len() == k as usize {
            self.res.push(self.tmp.clone());
            return;
        }

        if cur > n {
            return;
        }

        self.tmp.push(cur);
        self.dfs(cur + 1, n, k);
        self.tmp.pop();
        self.dfs(cur + 1, n, k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expect = vec![[2, 4].to_vec(),
                          [3, 4].to_vec(),
                          [2, 3].to_vec(),
                          [1, 2].to_vec(),
                          [1, 3].to_vec(),
                          [1, 4].to_vec()].sort();
        assert_eq!(expect, Solution::combine2(4, 2).sort());

        assert_eq!(vec![[1].to_vec()], Solution::combine2(1, 1));
    }
}