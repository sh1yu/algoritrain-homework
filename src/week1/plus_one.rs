// 66. 加一
// 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。
//
// 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
//
// 你可以假设除了整数 0 之外，这个整数不会以零开头。
//

use super::{Solution};

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        let mut new_digits = vec![0; digits.len() + 1];
        new_digits[0] = 1;
        new_digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
        println!("{:?}", Solution::plus_one(vec![0]));
        println!("{:?}", Solution::plus_one(vec![8, 9, 9]));
        println!("{:?}", Solution::plus_one(vec![9, 9, 9, 9]));
    }
}