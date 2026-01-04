// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

fn gcd(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}

impl Solution {
    // pub fn insert_greatest_common_divisors(
    //     mut head: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut ptr = &mut head;
    //     while ptr.as_ref().is_some() && ptr.as_ref().unwrap().next.is_some() {
    //         let p0 = ptr.as_mut().unwrap();
    //         p0.next = Some(Box::new(ListNode {
    //             val: gcd(p0.as_ref().val, p0.as_ref().next.as_ref().unwrap().val),
    //             next: p0.next.take(),
    //         }));
    //         ptr = &mut p0.as_mut().next.as_mut().unwrap().next;
    //     }
    //     head
    // }

    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut head) => {
                let mut next = head.next.take();
                let mut ptr = head.as_mut();
                while let Some(mut current) = next {
                    let current_ptr = current.as_mut() as *mut _;

                    next = current.next.take();
                    let val = gcd(ptr.val, current.val);
                    let inserted = Box::new(ListNode {
                        val,
                        next: Some(current),
                    });
                    ptr.next = Some(inserted);
                    ptr = unsafe { &mut *current_ptr };
                }
                Some(head)
            }
        }
    }
}
