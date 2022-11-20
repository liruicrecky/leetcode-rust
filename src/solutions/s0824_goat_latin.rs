/**
 * [824] Goat Latin
 *
 * You are given a string sentence that consist of words separated by spaces. Each word consists of lowercase and uppercase letters only.
 * We would like to convert the sentence to "Goat Latin" (a made-up language similar to Pig Latin.) The rules of Goat Latin are as follows:
 *
 * 	If a word begins with a vowel ('a', 'e', 'i', 'o', or 'u'), append "ma" to the end of the word.
 *
 * 		For example, the word "apple" becomes "applema".
 *
 *
 * 	If a word begins with a consonant (i.e., not a vowel), remove the first letter and append it to the end, then add "ma".
 *
 * 		For example, the word "goat" becomes "oatgma".
 *
 *
 * 	Add one letter 'a' to the end of each word per its word index in the sentence, starting with 1.
 *
 * 		For example, the first word gets "a" added to the end, the second word gets "aa" added to the end, and so on.
 *
 *
 *
 * Return the final sentence representing the conversion from sentence to Goat Latin.
 *  
 * Example 1:
 * Input: sentence = "I speak Goat Latin"
 * Output: "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
 * Example 2:
 * Input: sentence = "The quick brown fox jumped over the lazy dog"
 * Output: "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
 *  
 * Constraints:
 *
 * 	1 <= sentence.length <= 150
 * 	sentence consists of English letters and spaces.
 * 	sentence has no leading or trailing spaces.
 * 	All the words in sentence are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/goat-latin/
// discuss: https://leetcode.com/problems/goat-latin/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut res = String::new();
        let set: HashSet<char> = vec!['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U']
            .into_iter()
            .collect();

        let mut index: usize = 1;
        let mut words = sentence.split_whitespace();
        while let Some(word) = words.next() {
            let first = word.chars().next().unwrap();
            let mut new_word = String::new();
            if !set.contains(&first) {
                new_word.push_str(&word[1..]);
                new_word.push_str(&word[0..1]);
            } else {
                new_word.push_str(word);
            }
            res.push_str(&format!("{}{}{}{}", new_word, "ma", "a".repeat(index), " "));
            index += 1;
        }
        res.pop();
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_824() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
        );
        assert_eq!(Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()), "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string());
    }
}
