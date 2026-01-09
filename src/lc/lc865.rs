struct Solution;
use crate::lc_util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root, 0).1
    }
    fn dfs(root: Tree, depth: usize) -> (usize, Tree) {
        match root {
            Some(root) => {
                let borrowed = root.borrow();
                let left = Self::dfs(borrowed.left.clone(), depth + 1);
                let right = Self::dfs(borrowed.right.clone(), depth + 1);
                if left.0 == right.0 {
                    (left.0, Some(root.clone()))
                } else if left.0 < right.0 {
                    right
                } else {
                    left
                }
            }
            None => (depth, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let m1 = Some(5 as i32);
        let m2 = Some(5 as i32);
        let m3 = None;
        assert_eq!(m1.min(m3), None);
        assert_eq!(m3.min(m2), None);
        assert_eq!(m1.min(m2), Some(5));
    }
}
