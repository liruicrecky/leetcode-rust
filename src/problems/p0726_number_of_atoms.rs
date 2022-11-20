/**
 * [726] Number of Atoms
 *
 * Given a string formula representing a chemical formula, return the count of each atom.
 * The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.
 * One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow.
 *
 * 	For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.
 *
 * Two formulas are concatenated together to produce another formula.
 *
 * 	For example, "H2O2He3Mg4" is also a formula.
 *
 * A formula placed in parentheses, and a count (optionally added) is also a formula.
 *
 * 	For example, "(H2O2)" and "(H2O2)3" are formulas.
 *
 * Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1), followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.
 * The test cases are generated so that all the values in the output fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: formula = "H2O"
 * Output: "H2O"
 * Explanation: The count of elements are {'H': 2, 'O': 1}.
 *
 * Example 2:
 *
 * Input: formula = "Mg(OH)2"
 * Output: "H2MgO2"
 * Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.
 *
 * Example 3:
 *
 * Input: formula = "K4(ON(SO3)2)2"
 * Output: "K4N2O14S4"
 * Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
 *
 *  
 * Constraints:
 *
 * 	1 <= formula.length <= 1000
 * 	formula consists of English letters, digits, '(', and ')'.
 * 	formula is always valid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-atoms/
// discuss: https://leetcode.com/problems/number-of-atoms/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BTreeMap;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Debug)]
enum Tok {
    Num(usize),
    Name(String),
    Op(char),
}

impl Solution {
    fn count_of_atoms(formula: String) -> String {
        let mut it = formula.chars().peekable();
        let tokens = Self::parse_tokens(&mut it);
        let count: BTreeMap<String, usize> = Self::parse(&mut tokens.into_iter().peekable());
        count
            .into_iter()
            .map(|(k, v)| if v > 1 { format!("{}{}", k, v) } else { k })
            .collect()
    }

    fn parse(it: &mut Peekable<IntoIter<Tok>>) -> BTreeMap<String, usize> {
        let mut res: BTreeMap<String, usize> = BTreeMap::new();
        loop {
            match it.peek() {
                Some(Tok::Name(_)) => {
                    if let Some(Tok::Name(s)) = it.next() {
                        let x = if let Some(&Tok::Num(x)) = it.peek() {
                            it.next();
                            x
                        } else {
                            1
                        };
                        *res.entry(s).or_default() += x;
                    }
                }
                Some(&Tok::Op('(')) => {
                    it.next();
                    let inside = Self::parse(it);
                    it.next();
                    let x = if let Some(&Tok::Num(x)) = it.peek() {
                        it.next();
                        x
                    } else {
                        1
                    };
                    for (k, v) in inside {
                        *res.entry(k).or_default() += v * x;
                    }
                }
                Some(&Tok::Op(')')) | None => {
                    break;
                }
                _ => panic!(),
            }
        }
        res
    }

    fn parse_tokens(it: &mut Peekable<Chars>) -> Vec<Tok> {
        let mut res = vec![];
        while let Some(c) = it.next() {
            match c {
                '(' | ')' => res.push(Tok::Op(c)),
                '0'..='9' => {
                    let mut x = (c as u8 - b'0') as usize;
                    while let Some('0'..='9') = it.peek() {
                        x *= 10;
                        let y = (it.next().unwrap() as u8 - b'0') as usize;
                        x += y;
                    }
                    res.push(Tok::Num(x));
                }
                'A'..='Z' => {
                    let mut s = "".to_string();
                    s.push(c);
                    while let Some('a'..='z') = it.peek() {
                        s.push(it.next().unwrap());
                    }
                    res.push(Tok::Name(s));
                }
                _ => panic!(),
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
    fn test_726() {
        assert_eq!(
            Solution::count_of_atoms("H2O".to_string()),
            "H2O".to_string()
        );
        assert_eq!(
            Solution::count_of_atoms("Mg(OH)2".to_string()),
            "H2MgO2".to_string()
        );
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
            "K4N2O14S4".to_string()
        );
    }
}
