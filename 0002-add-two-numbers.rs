// I went with a recursive solution because honestly I'm scared of using .as_ref().
// I definitely found a weak spot in my understanding of Rust. 

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut x = Box::new(ListNode::new(0));

        fn normalize_val_and_return_next(x: &mut Box<ListNode>) -> Box<ListNode> {
            if x.val > 9 {
                x.val = x.val % 10;
                Box::new(ListNode::new(1))
            }
            else {
                Box::new(ListNode::new(0))
            }
        }

        fn add_two_num_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, x: &mut Box<ListNode>) -> bool {
            match (l1, l2) {
                (None, Some(v2)) => {
                    x.val += v2.val;
                    let mut y = normalize_val_and_return_next(x);
                    if add_two_num_recursive(None, v2.next, &mut y) || y.val > 0 {
                        x.next = Some(y);
                    }
                    true
                },
                (Some(v1), None) => {
                    x.val += v1.val;
                    let mut y = normalize_val_and_return_next(x);
                    if add_two_num_recursive(v1.next, None, &mut y) || y.val > 0 {
                        x.next = Some(y);
                    }
                    true
                },
                (Some(v1), Some(v2)) => {
                    x.val += v1.val + v2.val;
                    let mut y = normalize_val_and_return_next(x);
                    if add_two_num_recursive(v1.next, v2.next, &mut y) || y.val > 0 {
                        x.next = Some(y);
                    }
                    true
                },
                default => false
            }
        }

        add_two_num_recursive(l1, l2, &mut x);
        return Some(x);
    }
}
