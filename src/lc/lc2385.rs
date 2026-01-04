use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        traverse(root, 0, start)
            .visited
            .map(|(to_leaf, _, from_target)| to_leaf.max(from_target))
            .unwrap()
    }
}

struct Return {
    visited: Option<(i32, i32, i32)>,
    top_to_lowest_leaf: i32,
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, edges_from_top: i32, target: i32) -> Return {
    match root {
        None => Return {
            visited: None,
            top_to_lowest_leaf: edges_from_top - 1,
        },
        Some(node) => {
            let mut node = node.as_ref().borrow_mut();
            let left_return = traverse(node.left.take(), edges_from_top + 1, target);
            let right_return = traverse(node.right.take(), edges_from_top + 1, target);
            let top_to_lowest_leaf = left_return
                .top_to_lowest_leaf
                .max(right_return.top_to_lowest_leaf);

            if node.val == target {
                Return {
                    visited: Some((top_to_lowest_leaf - edges_from_top, 0, 0)),
                    top_to_lowest_leaf,
                }
            } else {
                let result = match left_return.visited {
                    None => match right_return.visited {
                        None => Return {
                            visited: None,
                            top_to_lowest_leaf,
                        },
                        Some((to_leaf, to_current, from_target)) => {
                            let to_current = to_current + 1;
                            Return {
                                visited: Some((
                                    to_leaf,
                                    to_current,
                                    from_target.max(
                                        to_current + left_return.top_to_lowest_leaf
                                            - edges_from_top,
                                    ),
                                )),
                                top_to_lowest_leaf,
                            }
                        }
                    },
                    Some((to_leaf, to_current, from_target)) => {
                        // assert!(right_return.visited.is_none());
                        let to_current = to_current + 1;
                        Return {
                            visited: Some((
                                to_leaf,
                                to_current,
                                from_target.max(
                                    to_current + right_return.top_to_lowest_leaf - edges_from_top,
                                ),
                            )),
                            top_to_lowest_leaf,
                        }
                    }
                };
                result
            }
        }
    }
}

#[test]
fn test_option_max_min() {
    println!("{:?}", Some(2).max(Some(1)));
    println!("{:?}", Some(2).max(Some(3)));
    println!("{:?}", None.max(Some(3)));
    println!("{:?}", Some(2).max(None));
}
