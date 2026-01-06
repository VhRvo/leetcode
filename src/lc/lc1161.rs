struct Solution;
use crate::lc_util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut maximal_sum = i32::MIN;
        let mut result_level = 1;
        let mut queue = vec![root];
        let mut level = 1;
        while !queue.is_empty() {
            let mut next_queue = Vec::with_capacity(queue.len());
            let mut sum = 0;
            let mut non_empty = false;
            for node in queue.into_iter() {
                if let Some(node) = node {
                    non_empty = true;
                    let borrowed = node.borrow();
                    sum += borrowed.val;
                    next_queue.push(borrowed.left.clone());
                    next_queue.push(borrowed.right.clone());
                }
            }
            if non_empty && sum > maximal_sum {
                result_level = level;
                maximal_sum = sum;
            }
            queue = next_queue;
            level += 1;
        }
        result_level
    }
}

mod tests {
    use super::*;
    use crate::lc_util::make_tree;
    use crate::{null_to_none, tree};

    #[test]
    fn test1() {
        // let root = make_tree(&[tree!(1), tree!(7), tree!(0), tree!(7), tree!(-8), tree!(-1), tree!(0)]);
        let root = make_tree(null_to_none![1, 7, 0, 7, -8, null, null]);
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    #[test]
    fn test2() {
        let root = make_tree(null_to_none![
            989,
            null,
            10250,
            98693,
            (-89388),
            null,
            null,
            null,
            (-32127)
        ]);
        assert_eq!(Solution::max_level_sum(root), 2);
    }
}
