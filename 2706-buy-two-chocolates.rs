impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        let sum: i32 = prices.iter().take(2).sum();
        if sum > money {
            return money;
        }
        money - sum
    }
}
