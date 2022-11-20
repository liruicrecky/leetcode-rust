/**
 * [449] Serialize and Deserialize BST
 *
 * Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary search tree. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.
 * The encoded string should be as compact as possible.
 *  
 * Example 1:
 * Input: root = [2,1,3]
 * Output: [2,1,3]
 * Example 2:
 * Input: root = []
 * Output: []
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	0 <= Node.val <= 10^4
 * 	The input tree is guaranteed to be a binary search tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/serialize-and-deserialize-bst/
// discuss: https://leetcode.com/problems/serialize-and-deserialize-bst/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i16,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i16) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::serialization(&root)
    }

    fn serialization(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();
        match root {
            None => ans,
            Some(node) => {
                let left_str = Self::serialization(&node.borrow().left);
                let right_str = Self::serialization(&node.borrow().right);
                ans.push_str(&format!(
                    "{} {} {}",
                    &node.borrow().val,
                    left_str,
                    right_str
                ));
                ans
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes: Vec<i16> = data
            .split_whitespace()
            .map(|x| x.parse::<i16>().unwrap())
            .collect();
        Self::deserialization(&nodes, 0, nodes.len() - 1)
    }
    
    fn deserialization(
        nodes: &Vec<i16>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end || nodes.len() == 0 {
            return None;
        }

        let val = nodes[start];
        let (mut s, mut e) = (start, end);
        while s < e {
            let mid = (s + e) >> 1;
            if nodes[mid] > val {
                e = mid
            } else {
                s = mid + 1;
            }
        }

        if nodes[e] <= val {
            e += 1;
        }

        let mut node = TreeNode::new(val as i32);
        node.left = Self::deserialization(nodes, start + 1, e - 1);
        node.right = Self::deserialization(nodes, e, end);
        Some(Rc::new(RefCell::new(node)))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_449() {
        assert_eq!(Codec::new().serialize(tree!(2, 1, 3)), "2 1   3  ");
        assert_eq!(
            Codec::new().deserialize("2 1   3  ".to_string()),
            tree!(2, 1, 3)
        );
    }
}
