impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        let m = img.len() as i32;
        let n = img[0].len() as i32;

        for i in 0..m {
            let mut row = vec![];
            for j in 0..n {
                let mut val = 0;
                let mut count = 0;

                for k in (i-1)..(i+2) {
                    for l in (j-1)..(j+2) {
                        if k < 0 || k >= m || l < 0 || l >= n {
                            continue;
                        }
                        val += img[k as usize][l as usize];
                        count += 1;
                    }
                }

                row.push(val / count);
            }
            answer.push(row);
        }

        answer
    }
}
