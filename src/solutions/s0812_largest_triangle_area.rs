/**
 * [812] Largest Triangle Area
 *
 * Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that can be formed by any three different points. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png" style="height: 369px; width: 450px;" />
 * Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
 * Output: 2.00000
 * Explanation: The five points are shown in the above figure. The red triangle is the largest.
 *
 * Example 2:
 *
 * Input: points = [[1,0],[0,0],[0,1]]
 * Output: 0.50000
 *
 *  
 * Constraints:
 *
 * 	3 <= points.length <= 50
 * 	-50 <= xi, yi <= 50
 * 	All the given points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-triangle-area/
// discuss: https://leetcode.com/problems/largest-triangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = f64::MIN;
        let n = points.len();
        let area = |x: &Vec<i32>, y: &Vec<i32>, z: &Vec<i32>| -> f64 {
            (0.5 * ((y[0] - x[0]) * (z[1] - x[1]) - (z[0] - x[0]) * (y[1] - x[1])) as f64).abs()
        };

        for i in 0..(n - 2) {
            for j in i + 1..(n - 1) {
                for k in j + 1..n {
                    max_area = max_area.max(area(&points[i], &points[j], &points[k]));
                }
            }
        }

        max_area
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_812() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.00000
        );
        assert_eq!(
            Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
            0.50000
        );
    }
}

