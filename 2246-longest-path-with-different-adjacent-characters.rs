// I'm happy with this solution! It's not the most elegant but it received 100% / 100%!

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let chars = s.chars().collect();
        let mut children: Vec<Vec<i32>> = vec![vec![]; n];
        for i in 1..parent.len() {
            children[parent[i] as usize].push(i as i32);
        }

        let mut possible_paths: Vec<i32> = vec![];
        let root_path = Self::dfs(0, &children, &chars, &mut possible_paths);
        possible_paths.push(root_path);

        match possible_paths.iter().max() {
            Some(i) => 1 + *i,
            None => 1
        }
    }

    // Returns the largest path of a node
    fn dfs(node: usize, children: &Vec<Vec<i32>>, chars: &Vec<char>, possible_paths: &mut Vec<i32>) -> i32 {
        let mut largest_path = 0;
        let mut second_largest = 0;
        let mut other_paths: Vec<i32> = vec![];
        for child in children[node as usize].iter() {
            let child = *child as usize;
            if chars[node] == chars[child] {
                other_paths.push(Self::dfs(child, children, chars, possible_paths));
            }
            else {
                let path = Self::dfs(child, children, chars, possible_paths) + 1;
                if path > largest_path {
                    second_largest = largest_path;
                    largest_path = path;
                }
                else if path > second_largest {
                    second_largest = path;
                }
            }
        }

        let largest_connecting = largest_path + second_largest;
        let largest_non_connecting = match other_paths.iter().max() {
            Some(i) => *i,
            None => 0
        };

        if largest_non_connecting > largest_connecting {
            possible_paths.push(largest_non_connecting);
        }
        else {
            possible_paths.push(largest_connecting);
        }

        largest_path
    }
}
