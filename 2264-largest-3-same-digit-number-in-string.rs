impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut last_c = '&';
        let mut instances = 0;
        let mut largest = None;
        
        num.chars().for_each(|c| {
            if c == last_c {
                instances += 1;
            }
            else {
                instances = 1;
            }

            last_c = c;

            if instances >= 3 {
                match largest {
                    Some(l) => {
                        if c > l {
                            largest = Some(c);
                        }
                    },
                    None => {
                        largest = Some(c);
                    }
                }
            }
        });

        match largest {
            None => "".to_string(),
            Some(l) => format!("{}{}{}", l, l, l)
        }
    }
}
