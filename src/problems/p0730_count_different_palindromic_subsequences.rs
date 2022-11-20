/**
 * [730] Count Different Palindromic Subsequences
 *
 * Given a string s, return the number of different non-empty palindromic subsequences in s. Since the answer may be very large, return it modulo 10^9 + 7.
 * A subsequence of a string is obtained by deleting zero or more characters from the string.
 * A sequence is palindromic if it is equal to the sequence reversed.
 * Two sequences a1, a2, ... and b1, b2, ... are different if there is some i for which ai != bi.
 *  
 * Example 1:
 *
 * Input: s = "bccb"
 * Output: 6
 * Explanation: The 6 different non-empty palindromic subsequences are 'b', 'c', 'bb', 'cc', 'bcb', 'bccb'.
 * Note that 'bcb' is counted only once, even though it occurs twice.
 *
 * Example 2:
 *
 * Input: s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
 * Output: 104860361
 * Explanation: There are 3104860382 different non-empty palindromic subsequences, which is 104860361 modulo 10^9 + 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s[i] is either 'a', 'b', 'c', or 'd'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-different-palindromic-subsequences/
// discuss: https://leetcode.com/problems/count-different-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let n = s.len();

        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }

        let s = s.as_bytes();
        for i in (0..(n - 1)).rev() {
            for j in i + 1..n {
                if s[i] != s[j] {
                    dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                } else {
                    dp[i][j] = dp[i + 1][j - 1] * 2 + 2;
                    let (mut l, mut r) = (i + 1, j - 1);
                    while l <= r && s[l] != s[i] {
                        l += 1;
                    }
                    while l <= r && s[r] != s[i] {
                        r -= 1;
                    }

                    if l == r {
                        dp[i][j] -= 1;
                    } else if l < r {
                        dp[i][j] -= 2 + dp[l + 1][r - 1];
                    }
                }

                if dp[i][j] >= 0 {
                    dp[i][j] = dp[i][j] % MOD;
                } else {
                    dp[i][j] = dp[i][j] + MOD;
                }
            }
        }

        dp[0][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_730() {
        assert_eq!(Solution::count_palindromic_subsequences("a".to_string()), 1);
        assert_eq!(
            Solution::count_palindromic_subsequences("abc".to_string()),
            3
        );
        assert_eq!(
            Solution::count_palindromic_subsequences("bccb".to_string()),
            6
        );

        assert_eq!(
            Solution::count_palindromic_subsequences(
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
            ),
            104860361
        );
    }
}
