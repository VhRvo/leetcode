use crate::lc_util::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Tree> {
        Solution::generate_trees_helper(1, n + 1)
    }

    fn generate_trees_helper(start: i32, end: i32) -> Vec<Tree> {
        if start == end {
            vec![None]
        } else {
            let mut result = Vec::new();
            for root in start..end {
                let left_enumerations = Solution::generate_trees_helper(start, root);
                let right_enumerations = Solution::generate_trees_helper(root + 1, end);
                let additional = left_enumerations.len() * right_enumerations.len();
                result.reserve(additional);
                for left in left_enumerations {
                    for right in right_enumerations.iter() {
                        // let val = root;
                        // let left = left.clone();
                        // let right = right.clone();
                        // result.push(Some(Rc::new(RefCell::new(TreeNode { val, left, right }))));
                        result.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: root,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test1() {}
}
