/**
 * [675] Cut Off Trees for Golf Event
 *
 * You are asked to cut off all the trees in a forest for a golf event. The forest is represented as an m x n matrix. In this matrix:
 *
 * 	0 means the cell cannot be walked through.
 * 	1 represents an empty cell that can be walked through.
 * 	A number greater than 1 represents a tree in a cell that can be walked through, and this number is the tree's height.
 *
 * In one step, you can walk in any of the four directions: north, east, south, and west. If you are standing in a cell with a tree, you can choose whether to cut it off.
 * You must cut off the trees in order from shortest to tallest. When you cut off a tree, the value at its cell becomes 1 (an empty cell).
 * Starting from the point (0, 0), return the minimum steps you need to walk to cut off all the trees. If you cannot cut off all the trees, return -1.
 * You are guaranteed that no two trees have the same height, and there is at least one tree needs to be cut off.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/trees1.jpg" style="width: 242px; height: 242px;" />
 * Input: forest = [[1,2,3],[0,0,4],[7,6,5]]
 * Output: 6
 * Explanation: Following the path above allows you to cut off the trees from shortest to tallest in 6 steps.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/trees2.jpg" style="width: 242px; height: 242px;" />
 * Input: forest = [[1,2,3],[0,0,0],[7,6,5]]
 * Output: -1
 * Explanation: The trees in the bottom row cannot be accessed as the middle row is blocked.
 *
 * Example 3:
 *
 * Input: forest = [[2,3,4],[0,0,5],[8,7,6]]
 * Output: 6
 * Explanation: You can follow the same path as Example 1 to cut off all the trees.
 * Note that you can cut off the first tree at (0, 0) before making any steps.
 *
 *  
 * Constraints:
 *
 * 	m == forest.length
 * 	n == forest[i].length
 * 	1 <= m, n <= 50
 * 	0 <= forest[i][j] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cut-off-trees-for-golf-event/
// discuss: https://leetcode.com/problems/cut-off-trees-for-golf-event/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (forest.len(), forest[0].len());

        let mut map: Vec<Vec<usize>> = vec![];
        for i in 0..n {
            for j in 0..m {
                if forest[i][j] > 1 {
                    map.push(vec![i, j]);
                }
            }
        }

        map.sort_by(|a, b| forest[a[0]][a[1]].cmp(&forest[b[0]][b[1]]));

        let dire: Vec<i8> = vec![1, 0, -1, 0, 1];
        let bfs = |x: usize, y: usize, dx: usize, dy: usize| -> i32 {
            if x == dx && y == dy {
                return 0;
            }

            let mut visited = vec![vec![false; m]; n];
            let mut que = VecDeque::new();
            let mut step = 0;

            que.push_back((x, y));
            visited[x][y] = true;
            while !que.is_empty() {
                let mut sz = que.len();
                step += 1;
                while sz > 0 {
                    let (sx, sy) = que.pop_front().unwrap();
                    sz -= 1;
                    for i in 0..4 {
                        let (xx, yy) = (sx as i8 + dire[i], sy as i8 + dire[i + 1]);
                        if xx < 0 || xx >= n as i8 || yy < 0 || yy >= m as i8 {
                            continue;
                        }
                        let (xx, yy) = (xx as usize, yy as usize);
                        if visited[xx][yy] || forest[xx][yy] == 0 {
                            continue;
                        }
                        if xx == dx && yy == dy {
                            return step;
                        }

                        visited[xx][yy] = true;
                        que.push_back((xx, yy));
                    }
                }
            }

            -1
        };

        let mut ans = 0;
        let (mut px, mut py) = (0, 0);
        for v in map.iter() {
            let next = bfs(px, py, v[0], v[1]);
            if next == -1 {
                ans = -1;
                break;
            }
            px = v[0];
            py = v[1];
            ans += next;
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_675() {
        assert_eq!(
            Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5],]),
            6
        );
        assert_eq!(
            Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5],]),
            -1
        );
        assert_eq!(
            Solution::cut_off_tree(vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6],]),
            6
        );
    }
}
