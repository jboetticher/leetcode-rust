impl Solution {
  pub fn largest_odd_number(num: String) -> String {
    let mut pop_count = 0;
    for c in num.chars().rev() {
      // Assumes that the num only consists of digits, otherwise match would be better
      if (c as u32 - '0' as u32) % 2 == 1 {
        break;
      }
      pop_count += 1;
    }

    if pop_count == num.len() {
      "".to_string()
    }
    else {
      (num[0..num.len() - pop_count]).to_string()
    }
  }
}
