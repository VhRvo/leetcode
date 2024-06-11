struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        type Tree = Rc<RefCell<TreeNode>>;
        fn process(queue: &mut VecDeque<(Tree, String)>, child: Option<Tree>, path: &str) -> bool {
            if let Some(child) = child {
                let mut path = path.to_owned();
                path.push_str("->");
                path.push_str(&child.borrow().val.to_string());
                queue.push_back((child, path));
                true
            } else {
                false
            }
        }

        match root {
            None => Vec::new(),
            Some(root) => {
                let mut result = Vec::new();
                let val = root.borrow().val;
                let mut queue = VecDeque::from([(root, val.to_string())]);
                while let Some((front, path)) = queue.pop_front() {
                    let left = front.borrow().left.clone();
                    let has_left = process(&mut queue, left, &path);

                    let right = front.borrow().right.clone();
                    let has_right = process(&mut queue, right, &path);

                    if !has_left && !has_right {
                        result.push(path);
                    }
                }

                result
            }
        }
    }
}
