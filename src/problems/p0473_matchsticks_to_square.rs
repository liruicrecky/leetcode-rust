/**
 * [473] Matchsticks to Square
 *
 * You are given an integer array matchsticks where matchsticks[i] is the length of the i^th matchstick. You want to use all the matchsticks to make one square. You should not break any stick, but you can link them up, and each matchstick must be used exactly one time.
 * Return true if you can make this square and false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: matchsticks = [1,1,2,2,2]
 * Output: true
 * Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
 *
 * Example 2:
 *
 * Input: matchsticks = [3,3,3,3,4]
 * Output: false
 * Explanation: You cannot find a way to form a square with all the matchsticks.
 *
 *  
 * Constraints:
 *
 * 	1 <= matchsticks.length <= 15
 * 	1 <= matchsticks[i] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/matchsticks-to-square/
// discuss: https://leetcode.com/problems/matchsticks-to-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use rand::{prelude::SliceRandom, Rng};
use std::cell::Cell;
impl Solution {
    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let (hi, lo, fa) = (1e4, 1e-4, 0.98);

        let mut m: u16 = 400;
        let ans = Cell::new(false);

        let sum: i32 = nums.iter().sum();
        if sum % 4 != 0 {
            return false;
        }

        let k = sum / 4;

        let calc = |nums: &Vec<i32>| {
            let mut diff = 0;
            let mut j: usize = 0;
            for _ in 0..4 {
                let mut cnt = 0;
                while j < n && cnt < k {
                    cnt += nums[j];
                    j += 1;
                }
                diff += (cnt - k).abs();
            }

            if diff == 0 {
                ans.set(true);
            }

            diff
        };

        let sa = |nums: &mut Vec<i32>| {
            let mut t: f64 = hi;
            let mut rng = rand::thread_rng();
            while t > lo && !ans.get() {
                let (a, b) = (rng.gen_range(0..n), rng.gen_range(0..n));
                let prev = calc(&nums);
                nums.swap(a, b);
                let cur = calc(&nums);
                let diff = (prev - cur) as f64 / t;
                let rng_f64: f64 = rng.gen();
                if diff.ln() > rng_f64 {
                    nums.swap(a, b);
                }
                t *= fa;
            }
        };

        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        while m != 0 {
            sa(&mut nums);
            m -= 1;
        }

        ans.get()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_473() {
        assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
        assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
        assert_eq!(
            Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            true
        );
        assert_eq!(
            Solution::makesquare(vec![
                403, 636, 824, 973, 815, 318, 881, 506, 863, 21, 834, 211, 316, 772, 803
            ]),
            true
        );
    }
}
