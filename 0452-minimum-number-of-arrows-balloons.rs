impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|x| x[0]);

        let mut arrows = 1;
        let mut higher_threshold = points[0][1];

        for i in 0..points.len() {
            if higher_threshold < points[i][0] {
                arrows += 1;
                higher_threshold = points[i][1];
            }
            else { 
                higher_threshold = std::cmp::min(higher_threshold, points[i][1]);
            }
        }

        arrows
    }
}
