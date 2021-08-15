// 49. 字母异位词分组
// 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
//
// 字母异位词 是由重新排列源单词的字母得到的一个新单词，所有源单词中的字母都恰好只用一次。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/group-anagrams

use super::Solution;
use std::collections::HashMap;

// 思路：使用数组作为hashmap的key进行分组
// 时间复杂度: O(nk)
// 空间复杂度: O(nk)
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        let mut group = HashMap::new();

        // 每一个字符串构造hash作为其key，并加入group中
        for str in strs {
            let mut hash = [0; 26];
            for c in str.chars() {
                hash[c as usize - 'a' as usize] += 1;
            }
            group.entry(hash).or_insert(vec![]).push(str);
        }

        // 将group中的值输出
        group.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::group_anagrams(vec!["eat".into(), "tea".into(), "tan".into(), "ate".into(), "nat".into(), "bat".into()]));
        println!("{:?}", Solution::group_anagrams(vec!["".into()]));
        println!("{:?}", Solution::group_anagrams(vec!["a".into()]));
    }
}