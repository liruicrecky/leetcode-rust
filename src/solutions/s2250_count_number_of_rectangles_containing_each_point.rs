/**
 * [2250] Count Number of Rectangles Containing Each Point
 *
 * You are given a 2D integer array rectangles where rectangles[i] = [li, hi] indicates that i^th rectangle has a length of li and a height of hi. You are also given a 2D integer array points where points[j] = [xj, yj] is a point with coordinates (xj, yj).
 * The i^th rectangle has its bottom-left corner point at the coordinates (0, 0) and its top-right corner point at (li, hi).
 * Return an integer array count of length points.length where count[j] is the number of rectangles that contain the j^th point.
 * The i^th rectangle contains the j^th point if 0 <= xj <= li and 0 <= yj <= hi. Note that points that lie on the edges of a rectangle are also considered to be contained by that rectangle.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/example1.png" style="width: 300px; height: 509px;" />
 * Input: rectangles = [[1,2],[2,3],[2,5]], points = [[2,1],[1,4]]
 * Output: [2,1]
 * Explanation:
 * The first rectangle contains no points.
 * The second rectangle contains only the point (2, 1).
 * The third rectangle contains the points (2, 1) and (1, 4).
 * The number of rectangles that contain the point (2, 1) is 2.
 * The number of rectangles that contain the point (1, 4) is 1.
 * Therefore, we return [2, 1].
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/example2.png" style="width: 300px; height: 312px;" />
 * Input: rectangles = [[1,1],[2,2],[3,3]], points = [[1,3],[1,1]]
 * Output: [1,3]
 * Explanation:
 * The first rectangle contains only the point (1, 1).
 * The second rectangle contains only the point (1, 1).
 * The third rectangle contains the points (1, 3) and (1, 1).
 * The number of rectangles that contain the point (1, 3) is 1.
 * The number of rectangles that contain the point (1, 1) is 3.
 * Therefore, we return [1, 3].
 *
 *  
 * Constraints:
 *
 * 	1 <= rectangles.length, points.length <= 5 * 10^4
 * 	rectangles[i].length == points[j].length == 2
 * 	1 <= li, xj <= 10^9
 * 	1 <= hi, yj <= 100
 * 	All the rectangles are unique.
 * 	All the points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/
// discuss: https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        const N: usize = 101;
        let mut col_vec = vec![vec![]; N];
        for r in rectangles.iter() {
            col_vec[r[1] as usize].push(r[0]);
        }

        for r in col_vec.iter_mut() {
            r.sort();
        }

        let bi_serach = |x: &i32, v: &Vec<i32>| {
            let mut size = v.len();
            let mut left = 0;
            let mut right = size;
            while left < right {
                let mid = left + size / 2;
                if x > &v[mid] {
                    left = mid + 1;
                } else if x < &v[mid] {
                    right = mid;
                } else {
                    return mid;
                }

                size = right - left;
            }

            left
        };

        let mut count = vec![0; points.len()];
        for i in 0..points.len() {
            let (x, y) = (points[i][0], points[i][1] as usize);
            for j in y..N {
                if !col_vec[j].is_empty() {
                    let key = bi_serach(&x, &col_vec[j]) as i32;
                    count[i] += col_vec[j].len() as i32 - key;
                }
            }
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2250() {
        assert_eq!(
            Solution::count_rectangles(
                vec![vec![1, 2], vec![2, 3], vec![2, 5]],
                vec![vec![2, 1], vec![1, 4]]
            ),
            vec![2, 1]
        );
        assert_eq!(
            Solution::count_rectangles(
                vec![vec![1, 1], vec![2, 2], vec![3, 3]],
                vec![vec![1, 3], vec![1, 1]]
            ),
            vec![1, 3]
        );
    }
}
