impl Solution {
    // Logical
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }

    // Recursive
    /*
    pub fn number_of_matches(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }

        let matches = n / 2;
        if n % 2 == 1 {
            return matches + Self::number_of_matches(matches + 1);
        }

        matches + Self::number_of_matches(matches)
    }
    */
}
