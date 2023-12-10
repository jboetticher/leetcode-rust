use std::collections::HashMap;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = vec![];

        // You can continuously add conditions here (7 for Bazz, 13 for Bozz, etc)
        let mut conditions = HashMap::new();
        conditions.insert(3, "Fizz");
        conditions.insert(5, "Buzz");

        let mut condition_keys: Vec<i32> = conditions.keys().cloned().collect::<Vec<i32>>();
        condition_keys.sort();

        for i in 1..n+1 {
            let mut response = "".to_string();
            for c in condition_keys.iter() {
                if i % c == 0 {
                    response += conditions.get(c).unwrap_or(&"");
                }
            }

            if response == "" {
                answer.push(i.to_string());
            }
            else {
                answer.push(response);
            }
        }

        answer
    }
}
