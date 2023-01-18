impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Kadene's algorithm
        let kadene = nums.iter()
            .fold(Kadene { 
                local_max: 0, global_max: i32::MIN, local_min: 0, global_min: i32::MAX, total_sum: 0, all_negative: true
            }, 
            |mut k, x| {
                k.local_max = i32::max(*x, *x + k.local_max);
                if k.local_max > k.global_max {
                    k.global_max = k.local_max;
                }

                k.local_min = i32::min(*x, *x + k.local_min);
                if k.local_min < k.global_min {
                    k.global_min = k.local_min;
                }

                k.total_sum += *x;
                k.all_negative = k.all_negative && *x <= 0;

                k
            }
        );

        if kadene.all_negative {
            *nums.iter().max().unwrap()
        }
        else {
            i32::max(kadene.global_max, kadene.total_sum - kadene.global_min)
        }
        // kadene.global_max
    }
}

struct Kadene {
    local_max: i32,
    global_max: i32,
    local_min: i32,
    global_min: i32,
    total_sum: i32,
    all_negative: bool
}
