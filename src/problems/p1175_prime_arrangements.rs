/**
 * [1175] Prime Arrangements
 *
 * Return the number of permutations of 1 to n so that prime numbers are at prime indices (1-indexed.)
 * (Recall that an integer is prime if and only if it is greater than 1, and cannot be written as a product of two positive integers both smaller than it.)
 * Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 12
 * Explanation: For example [1,2,5,4,3] is a valid permutation, but [5,2,3,4,1] is not because the prime number 5 is at index 1.
 *
 * Example 2:
 *
 * Input: n = 100
 * Output: 682289015
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prime-arrangements/
// discuss: https://leetcode.com/problems/prime-arrangements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let check_prime = |x: i32| {
            if (x & 1) == 0 {
                return false;
            }

            let mut i = 2;
            while i * i <= x {
                if x % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        };

        let (mut cnt_prime, mut cnt_not_prime) = (1, 1);
        let (mut prime_comb, mut not_prime_comb) = (1 as i64, 1 as i64);
        let mut i = 3;

        while i <= n {
            if check_prime(i) {
                cnt_prime += 1;
                prime_comb = prime_comb % MOD * cnt_prime as i64;
            } else {
                cnt_not_prime += 1;
                not_prime_comb = not_prime_comb % MOD * cnt_not_prime as i64;
            }
            i += 1;
        }

        ((prime_comb % MOD) * (not_prime_comb % MOD) % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1175() {
        assert_eq!(Solution::num_prime_arrangements(5), 12);
        assert_eq!(Solution::num_prime_arrangements(100), 682289015);
    }
}
