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
use std::rc::Rc;
// impl Solution {
//     pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         let root = root.as_ref().unwrap().borrow();
//         let left = root.left.clone();
//         let left = left.as_ref().unwrap().borrow();
//         let right = root.right.clone();
//         let right = right.as_ref().unwrap().borrow();
//
//         // root.val == left.val + right.val
//     }
// }
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map_or(false, |root| {
            root.borrow().left.as_ref().map_or(false, |left| {
                root.borrow().right.as_ref().map_or(false, |right| {
                    right.borrow().val + left.borrow().val == root.borrow().val
                })
            })
        })
    }
}
// impl Solution {
//     pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         root.map_or(false, |root| {
//           let root = root.borrow();
//             root.left.clone().map_or(false, |left| {
//                 root.right.clone().map_or(false, |right| {
//                     right.borrow().val + left.borrow().val == root.val
//                 })
//             })
//         })
//     }
// }
// impl Solution {
//     pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         match root {
//             Some(root) => {
//                 let root = root.borrow();
//                 if let (Some(left), Some(right)) = (&root.left, &root.right) {
//                     root.val == left.borrow().val + right.borrow().val
//                 } else {
//                     false
//                 }
//             }
//             None => true,
//         }
//     }
// }
