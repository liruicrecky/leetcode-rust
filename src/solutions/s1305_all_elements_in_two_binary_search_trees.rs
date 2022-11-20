/**
 * [1305] All Elements in Two Binary Search Trees
 *
 * Given two binary search trees root1 and root2, return a list containing all the integers from both trees sorted in ascending order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/18/q2-e1.png" style="width: 457px; height: 207px;" />
 * Input: root1 = [2,1,4], root2 = [1,0,3]
 * Output: [0,1,1,2,3,4]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/18/q2-e5-.png" style="width: 352px; height: 197px;" />
 * Input: root1 = [1,null,8], root2 = [8,1]
 * Output: [1,1,8,8]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each tree is in the range [0, 5000].
 * 	-10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
// discuss: https://leetcode.com/problems/all-elements-in-two-binary-search-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::rc::Rc;
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::in_order(&root1, &mut ans);
        Self::in_order(&root2, &mut ans);
        ans.sort();

        ans
    }

    fn in_order(r: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = r {
            Self::in_order(&node.borrow().left, v);
            v.push(node.borrow().val);
            Self::in_order(&node.borrow().right, v);
        }
    }

    fn pre_order(r: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = r {
            v.push(node.borrow().val);
            Self::pre_order(&node.borrow().left, v);
            Self::pre_order(&node.borrow().right, v);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1305() {
        assert_eq!(
            Solution::get_all_elements(tree![2, 1, 4], tree![1, 0, 3]),
            vec![0, 1, 1, 2, 3, 4]
        );
        assert_eq!(
            Solution::get_all_elements(tree![1, null, 8], tree![8, 1]),
            vec![1, 1, 8, 8]
        );
    }
}
