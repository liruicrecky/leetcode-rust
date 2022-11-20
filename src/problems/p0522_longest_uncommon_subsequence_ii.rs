/**
 * [522] Longest Uncommon Subsequence II
 *
 * Given an array of strings strs, return the length of the longest uncommon subsequence between them. If the longest uncommon subsequence does not exist, return -1.
 * An uncommon subsequence between an array of strings is a string that is a subsequence of one string but not the others.
 * A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.
 *
 * 	For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "aebdc" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).
 *
 *  
 * Example 1:
 * Input: strs = ["aba","cdc","eae"]
 * Output: 3
 * Example 2:
 * Input: strs = ["aaa","aaa","aa"]
 * Output: -1
 *  
 * Constraints:
 *
 * 	2 <= strs.length <= 50
 * 	1 <= strs[i].length <= 10
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-uncommon-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let check = |a: &str, b: &str| {
            let (n1, n2) = (a.len(), b.len());
            if n1 > n2 {
                return false;
            }

            let (mut i, mut j) = (0, 0);
            while i < n1 && j < n2 {
                if a.chars().nth(i).unwrap() == b.chars().nth(j).unwrap() {
                    i += 1;
                }
                j += 1;
            }
            i == n1
        };

        let n = strs.len();
        let mut res: i32 = -1;
        for i in 0..n {
            let mut find = true;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if check(&strs[i], &strs[j]) {
                    find = false;
                    break;
                }
            }

            if find {
                res = res.max(strs[i].len() as i32);
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_522() {
        assert_eq!(
            Solution::find_lu_slength(vec![
                String::from("aba"),
                String::from("cdc"),
                String::from("eae")
            ]),
            3
        );
        assert_eq!(
            Solution::find_lu_slength(vec![
                String::from("aaa"),
                String::from("aaa"),
                String::from("aa")
            ]),
            -1
        );
    }
}
