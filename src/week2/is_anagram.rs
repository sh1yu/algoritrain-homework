// 242. 有效的字母异位词
// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
//
// 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
//
// 提示:
//
// 1 <= s.length, t.length <= 5 * 104
// s 和 t 仅包含小写字母
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-anagram

use super::Solution;


// 思路：直接使用字符数组作为hash key进行字符的计数
// 时间复杂度： O(n)
// 空间复杂度： O(n)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 使用26位数组记录26个字母的出现次数
        let mut count = [0; 26];
        // s数组计数
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        // t数组消耗计数
        for c in t.chars() {
            count[c as usize - 'a' as usize] -= 1;
        }
        // 所有字母均被恰好消耗，说明为字母异位词
        count.iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?} -- expect true", Solution::is_anagram("anagram".into(), "nagaram".into()));
        println!("{:?} -- expect false", Solution::is_anagram("rat".into(), "car".into()));
    }
}