/**
 * [515] Find Largest Value in Each Tree Row
 *
 * Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/21/largest_e1.jpg" style="width: 300px; height: 172px;" />
 * Input: root = [1,3,2,5,3,null,9]
 * Output: [1,3,9]
 *
 * Example 2:
 *
 * Input: root = [1,2,3]
 * Output: [1,3]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [0, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// discuss: https://leetcode.com/problems/find-largest-value-in-each-tree-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }

        // let mut que: VecDeque<Option<Rc<RefCell<TreeNode>>>> = vec![root].into_iter().collect();
        let mut que = VecDeque::new();
        que.push_back(root);
        while !que.is_empty() {
            let n = que.len();
            ans.push(
                (0..n)
                    .into_iter()
                    .map(|_| {
                        let mut node = que.pop_front().unwrap();
                        let mut node = node.as_mut().unwrap().borrow_mut();
                        if node.left.is_some() {
                            que.push_back(node.left.take());
                        }
                        if node.right.is_some() {
                            que.push_back(node.right.take());
                        }
                        node.val
                    })
                    .fold(i32::MIN, |mut max_val, val| {
                        max_val = max_val.max(val);
                        max_val
                    }),
            );
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_515() {
        assert_eq!(
            Solution::largest_values(tree!(1, 3, 2, 5, 3, null, 9)),
            vec![1, 3, 9]
        );
        assert_eq!(Solution::largest_values(tree!(1, 2, 3)), vec![1, 3]);
    }
}
