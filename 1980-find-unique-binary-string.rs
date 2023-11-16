impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        /* 
        Everybody in the comments are freaking out about Cantor, so I guess I'll try it
        Unfortunately the theorem does not come intuitive to me whatsoever
        https://en.wikipedia.org/wiki/Cantor%27s_diagonal_argument
        */

        /* Optimization Info:

        This line is far more efficient than...
        let c: char = nums[0].as_bytes()[1] as char;

        This line, because this line is an iterator. They do the same thing.
        let c: char = nums[0].chars().nth(1).unwrap();

        */

        let n = nums.len();
        let mut answer = String::with_capacity(n);
        for i in 0..n {
            if nums[i].as_bytes()[i] as char == '1' {
                answer.push('0');
                continue;
            }
            answer.push('1');
        }

        answer
    }
}
