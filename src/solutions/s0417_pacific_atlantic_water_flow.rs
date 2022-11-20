/**
 * [417] pacific Atlantic Water Flow
 *
 * There is an m x n rectangular island that borders both the pacific Ocean and Atlantic Ocean. The pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
 * The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
 * The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
 * Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the pacific and Atlantic oceans.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg" style="width: 573px; height: 573px;" />
 * Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
 * Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
 *
 * Example 2:
 *
 * Input: heights = [[2,1],[1,2]]
 * Output: [[0,0],[0,1],[1,0],[1,1]]
 *
 *  
 * Constraints:
 *
 * 	m == heights.length
 * 	n == heights[r].length
 * 	1 <= m, n <= 200
 * 	0 <= heights[r][c] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pacific-atlantic-water-flow/
// discuss: https://leetcode.com/problems/pacific-atlantic-water-flow/discuss/?currentpage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let (n, m) = (matrix.len(), matrix[0].len());
        let (mut p, mut a) = (vec![vec![false; m]; n], vec![vec![false; m]; n]);
        let dire: Vec<i8> = vec![0, 1, 0, -1, 0];

        for i in 0..n {
            for j in 0..m {
                if !p[i][j] && (i == 0 || j == 0) {
                    Self::dfs(i, j, &matrix, &mut p, &dire);
                }

                if !a[i][j] && (i == n - 1 || j == m - 1) {
                    Self::dfs(i, j, &matrix, &mut a, &dire);
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if p[i][j] && a[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }

    fn dfs(x: usize, y: usize, matrix: &[Vec<i32>], visited: &mut [Vec<bool>], dire: &Vec<i8>) {
        if !visited[x][y] {
            visited[x][y] = true;
            for i in 0..4 {
                let (xx, yy) = (x as i8 + dire[i], y as i8 + dire[i + 1]);
                if xx < 0 || xx >= matrix.len() as i8 || yy < 0 || yy >= matrix[0].len() as i8 {
                    continue;
                }
                let (xx, yy) = (xx as usize, yy as usize);
                if matrix[xx][yy] >= matrix[x][y] {
                    Self::dfs(xx, yy, matrix, visited, dire);
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_417() {
        assert_eq!(
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
        );
        assert_eq!(
            Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]]),
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]
        );
    }
}
