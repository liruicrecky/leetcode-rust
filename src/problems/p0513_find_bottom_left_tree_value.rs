/**
 * [513] Find Bottom Left Tree Value
 *
 * Given the root of a binary tree, return the leftmost value in the last row of the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree1.jpg" style="width: 302px; height: 182px;" />
 * Input: root = [2,1,3]
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree2.jpg" style="width: 432px; height: 421px;" />
 * Input: root = [1,2,3,4,null,5,6,null,null,7]
 * Output: 7
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-bottom-left-tree-value/
// discuss: https://leetcode.com/problems/find-bottom-left-tree-value/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut que = VecDeque::new();
        que.push_back(root);

        while !que.is_empty() {
            let n = que.len();
            for i in 0..n {
                let mut node = que.pop_front().unwrap();
                let mut node = node.as_mut().unwrap().borrow_mut();
                if node.left.is_some() {
                    que.push_back(node.left.take());
                }
                if node.right.is_some() {
                    que.push_back(node.right.take());
                }

                if i == 0 {
                    ans = node.val;
                }
            }
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_513() {
        assert_eq!(Solution::find_bottom_left_value(tree!(2, 1, 3)), 1);
        assert_eq!(
            Solution::find_bottom_left_value(tree!(1, 2, 3, 4, null, 5, 6, null, null, 7)),
            7
        );
    }
}
