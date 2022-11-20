/**
 * [875] Koko Eating Bananas
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has piles[i] bananas. The guards have gone and will come back in h hours.
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
 * Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
 * Return the minimum integer k such that she can eat all the bananas within h hours.
 *  
 * Example 1:
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 * Example 2:
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 * Example 3:
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *  
 * Constraints:
 *
 * 	1 <= piles.length <= 10^4
 * 	piles.length <= h <= 10^9
 * 	1 <= piles[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/koko-eating-bananas/
// discuss: https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let calc = |x: i32| {
            piles.iter().fold(0, |mut times, &pile| {
                times += (pile - 1) / x + 1;
                times
            })
        };

        let (mut l, mut r) = (0, 1e9 as usize);
        while l <= r {
            let mid = l + (r - l) / 2;
            if calc(mid as i32) <= h {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        l as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_875() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
