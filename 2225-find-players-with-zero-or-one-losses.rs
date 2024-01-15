use std::collections::HashSet;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut all_players, mut defeated_once, mut defeated_more_than_once) = 
            (HashSet::new(), HashSet::new(), HashSet::new());
        for m in matches {
            // Acknowledge that both players exist
            all_players.insert(m[0]);
            all_players.insert(m[1]);

            if (defeated_once.contains(&m[1])) {
                defeated_more_than_once.insert(m[1]);
            }
            else {
                defeated_once.insert(m[1]);
            }
        }

        let mut once_losers: Vec<i32> = defeated_once
            .difference(&defeated_more_than_once)
            .into_iter()
            .cloned()
            .collect();
        let mut pure_winners: Vec<i32> = all_players
            .difference(&defeated_once)
            .into_iter()
            .cloned()
            .collect();
        once_losers.sort_unstable();
        pure_winners.sort_unstable();

        return vec![pure_winners, once_losers];
    }
}
