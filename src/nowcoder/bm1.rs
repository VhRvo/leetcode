#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param head ListNode类
     * @return ListNode类
     */
    pub fn reverse_list(&self, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut acc = None;
        while let Some(mut current) = head {
            head = current.next;
            current.next = acc;
            acc = Some(current);
        }
        acc
    }
}
