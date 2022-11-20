/**
 * [8] String to Integer (atoi)
 *
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
 * The algorithm for myAtoi(string s) is as follows:
 *
 * 	Read in and ignore any leading whitespace.
 * 	Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
 * 	Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
 * 	Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
 * 	If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -2^31 should be clamped to -2^31, and integers greater than 2^31 - 1 should be clamped to 2^31 - 1.
 * 	Return the integer as the final result.
 *
 * Note:
 *
 * 	Only the space character ' ' is considered a whitespace character.
 * 	Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 *
 *  
 * Example 1:
 *
 * Input: s = "42"
 * Output: 42
 * Explanation: The underlined characters are what is read in, the caret is the current reader position.
 * Step 1: "42" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "42" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "42" ("42" is read in)
 *            ^
 * The parsed integer is 42.
 * Since 42 is in the range [-2^31, 2^31 - 1], the final result is 42.
 *
 * Example 2:
 *
 * Input: s = "   -42"
 * Output: -42
 * Explanation:
 * Step 1: "   -42" (leading whitespace is read and ignored)
 *             ^
 * Step 2: "   -42" ('-' is read, so the result should be negative)
 *              ^
 * Step 3: "   -42" ("42" is read in)
 *                ^
 * The parsed integer is -42.
 * Since -42 is in the range [-2^31, 2^31 - 1], the final result is -42.
 *
 * Example 3:
 *
 * Input: s = "4193 with words"
 * Output: 4193
 * Explanation:
 * Step 1: "4193 with words" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
 *              ^
 * The parsed integer is 4193.
 * Since 4193 is in the range [-2^31, 2^31 - 1], the final result is 4193.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 200
 * 	s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn my_atoi(s: String) -> i32 {
        let mut start = s.trim_start();
        let mut res: i32 = 0;
        let mut positive = true;
        if start.len() > 1 {
            let c = &start[0..1];
            match c {
                "+" => {
                    start = &start[1..];
                }
                "-" => {
                    start = &start[1..];
                    positive = false;
                }
                _ => {
                    if let Some(c) = c.chars().next() {
                        if !('0'..='9').contains(&c) {
                            return 0;
                        }
                    }
                }
            }
        }
        for c in start.chars() {
            if ('0'..='9').contains(&c) {
                res = match res.checked_mul(10) {
                    None => {
                        return Self::overflow(positive);
                    }
                    Some(val) => val,
                };
                res = match res.checked_add((c as u8 - b'0') as i32) {
                    None => {
                        return Self::overflow(positive);
                    }
                    Some(val) => val,
                };
            } else {
                break;
            }
        }
        if !positive {
            res = match res.checked_mul(-1) {
                None => {
                    return Self::overflow(positive);
                }
                Some(val) => val,
            };
        }
        res
    }

    fn overflow(positive: bool) -> i32 {
        if positive {
            i32::MAX
        } else {
            i32::MIN
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
