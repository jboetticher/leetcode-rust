// One of the worst performing solutions out there, but I wanted to practice
// using iterators. I guess I did that, but holy crap I should have used
// arrays like everyone else

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut vecs: Vec<Vec<char>> = vec![];
        
        for s in strs {
            if s.len() > vecs.len() {
                for i in 0..s.len() {
                    vecs.push(vec![]);
                }
            }
            vecs = s.chars().fold((0, vecs), |mut i, c| {
                i.1[i.0 as usize].push(c);
                (i.0 + 1, i.1)
            }).1;
        }

        (vecs.len() - vecs.iter().filter(|s| { 
            s.iter().fold((true, 'a'), |tup, c| (tup.1 <= *c && tup.0, *c)).0
        }).count()) as i32
    }
}
