struct Solution;


use crate::lc_util::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
const MODULO: i64 = 1e9 as i64 + 7;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memo = HashSet::new();
        let sum = Self::tree_sum(&mut memo, root);
        let mut result = 0;
        for &subtree_sum in memo.iter() {
            result = result.max((sum - subtree_sum) * subtree_sum);
        }
        (result % MODULO) as i32
    }
    fn tree_sum(memo: &mut HashSet<i64>, root: Option<Rc<RefCell<TreeNode>>>) -> i64 {
        let sum = if let Some(root) = root {
            let borrowed = root.borrow();
            let left_sum = Self::tree_sum(memo, borrowed.left.clone());
            let right_sum = Self::tree_sum(memo, borrowed.right.clone());
            left_sum + borrowed.val as i64 + right_sum
        } else {
            0
        };
        memo.insert(sum);
        sum
    }
}
