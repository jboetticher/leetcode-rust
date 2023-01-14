// This solution was mostly from Leetcode the Hard Way. I had never heard of Union Find before this!

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        // Every character is initially their own smallest
        let mut root: [i32; 26] = [0; 26];
        for i in 0..26 {
            root[i] = i as i32;
        }

        // Get root element
        fn get(root: &mut [i32; 26], x: i32) -> i32 {
            if x == root[x as usize] {
                return x;
            }
            root[x as usize] = get(root, root[x as usize]);
            root[x as usize]
        }
        
        // Union find
        fn unite(root: &mut [i32; 26], x: i32, y: i32) {
            let x = get(root, x);
            let y = get(root, y);

            // if their roots are not same, we combine them
            if x != y {
                // smaller first
                if x < y {
                    root[y as usize] = x;
                } else {
                    root[x as usize] = y;
                }
            }
        }

        let mut ans = String::new();

        // Optimize each letter through union find
        let char_shift = 'a' as i32;
        for i in 0..s1.len() {
            unite(&mut root, s1.as_bytes()[i] as i32 - char_shift, s2.as_bytes()[i] as i32 - char_shift);
        }

        // Build from the optimized
        for i in base_str.bytes() {
            ans.push((get(&mut root, i as i32 - char_shift) + char_shift) as u8 as char);
        }
        ans
    }
}
