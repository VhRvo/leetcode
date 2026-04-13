use crate::lc_util::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn inefficient_rob(root: Tree, available: bool) -> i32 {
    match root {
        None => 0,
        Some(root) => {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            let skipped =
                inefficient_rob(left.clone(), true) + inefficient_rob(right.clone(), true);
            let robbed =
                root.borrow().val + inefficient_rob(left, false) + inefficient_rob(right, false);
            if available {
                skipped.max(robbed)
            } else {
                skipped
            }
        }
    }
}

fn rob(root: Tree) -> (i32, i32) {
    match root {
        None => (0, 0),
        Some(root) => {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            let (l_robbed, l_skipped) = rob(left);
            let (r_robbed, r_skipped) = rob(right);
            let skipped = l_robbed + r_robbed;
            let robbed = root.borrow().val + l_skipped + r_skipped;
            (skipped.max(robbed), skipped)
        }
    }
}

impl Solution {
    pub fn rob(root: Tree) -> i32 {
        rob(root).0
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test1() {}
}
