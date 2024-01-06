use std::cmp::max;

#[derive(Debug)]
struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32
}

impl Job {
    pub fn new(data: ((&i32, &i32), &i32)) -> Job {
        Job {
            start_time: *data.0.0,
            end_time: *data.0.1,
            profit: *data.1
        }
    }
}

/*

My first "dp" solution was O(n^2), which worked, but it was far too inefficient.

This solution is also quite inefficient, and was only barely fast enough, but it was a 
solution that I found myself, so that made me happy. I may have made the most inefficient
accepted solution on LeetCode.

I particularly liked this solution when looking at others, as it runs along some of the same
lines that I found with the dp, but without the vestigial stupidness of my original O(n^2)
solution.

https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2849785/rust-solution/

The only reason I'm putting this here is because of the gnarly iterators that I made up, 
which I'm proud of. Of course, only the inexperienced appreciate complexity. Which must mean
I am inexperienced.

*/

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs_sorted_by_start: Vec<Job> = start_time.iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(Job::new)
            .collect();
        jobs_sorted_by_start.sort_unstable_by(|a, b| a.start_time.cmp(&b.start_time));

        // dp[i] is the maximum profit taking elements starting from [i]
        // i guess we can go backwards then?
        let mut dp = vec![0; n];

        for i in (0..n).rev() {
            let cur_job: &Job = &jobs_sorted_by_start[i];

            let mut max_profit = cur_job.profit + 
                (i..n)
                    .filter_map(|k| if jobs_sorted_by_start[k].start_time >= cur_job.end_time { Some(dp[k]) } else { None })
                    .take(1)
                    .next()
                    .unwrap_or(0);
            
            if i + 1 < n {
                dp[i] = max(max_profit, dp[i + 1]);
            }
            else {
                dp[i] = max_profit;
            }
        }

        *dp.iter().max().unwrap()
    }
}
