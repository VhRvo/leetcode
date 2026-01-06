use std::cell::RefCell;
use std::rc::Rc;

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

/// 从层序遍历数组构建二叉树
/// 使用 Some(val) 表示节点值,None 表示 null 节点
pub fn make_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < values.len() && !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        // 处理左子节点
        if i < values.len() {
            if let Some(val) = values[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;
        }

        // 处理右子节点
        if i < values.len() {
            if let Some(val) = values[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}

#[macro_export]
macro_rules! tree {
    (null) => {
        None
    };
    (- $val:tt) => {
        Some(-$val)
    };
    (($val:expr)) => {
        Some($val)
    };
    ($val:tt) => {
        Some($val)
    };
}

// treeify![1, -8, null, 3]
// => vec![tree!(1), tree!(- 8), tree!(null), tree!(3)]
// => vec![Some(1), Some(-8), None, Some(3)]
#[macro_export]
#[warn(unused_mut)]
macro_rules! null_to_none {
    // 处理空列表
    () => {
        vec![]
    };
    // 处理负数: -num, rest...
    (- $num:tt $(, $($rest:tt)*)?) => {
        {
            let mut v = vec![tree!(- $num)];
            $(v.extend(null_to_none![$($rest)*]);)?
            v
        }
    };
    // 处理单个 token 后面跟逗号和其余部分
    ($val:tt $(, $($rest:tt)*)?) => {
        {
            let mut v = vec![tree!($val)];
            $(v.extend(null_to_none![$($rest)*]);)?
            v
        }
    };
}
