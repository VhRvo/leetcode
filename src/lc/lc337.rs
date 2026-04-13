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
            let not_choose_root =
                inefficient_rob(left.clone(), true) + inefficient_rob(right.clone(), true);
            let choose_root =
                root.borrow().val + inefficient_rob(left, false) + inefficient_rob(right, false);
            if available {
                not_choose_root.max(choose_root)
            } else {
                not_choose_root
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
            let left_result = rob(left);
            let right_result = rob(right);
            let not_choose_root = left_result.0 + right_result.0;
            let choose_root = root.borrow().val + left_result.1 + right_result.1;
            (not_choose_root.max(choose_root), not_choose_root)
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
