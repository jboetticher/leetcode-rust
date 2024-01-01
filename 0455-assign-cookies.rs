impl Solution {
    pub fn find_content_children(mut children: Vec<i32>, mut cookies: Vec<i32>) -> i32 {
        children.sort_unstable();
        cookies.sort_unstable();

        let (mut child_ptr, mut cookie_ptr) = (0, 0);
        while cookie_ptr < cookies.len() && child_ptr < children.len() {
            if cookies[cookie_ptr] < children[child_ptr] {
                cookie_ptr += 1;
            }
            else if cookies[cookie_ptr] >= children[child_ptr] {
                child_ptr += 1;
                cookie_ptr += 1;
            }
        }

        child_ptr as i32
    }
}
