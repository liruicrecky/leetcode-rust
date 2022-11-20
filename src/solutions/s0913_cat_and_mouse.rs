/**
 * [913] Cat and Mouse
 *
 * A game on an undirected graph is played by two players, Mouse and Cat, who alternate turns.
 * The graph is given as follows: graph[a] is a list of all nodes b such that ab is an edge of the graph.
 * The mouse starts at node 1 and goes first, the cat starts at node 2 and goes second, and there is a hole at node 0.
 * During each player's turn, they must travel along one edge of the graph that meets where they are.  For example, if the Mouse is at node 1, it must travel to any node in graph[1].
 * Additionally, it is not allowed for the Cat to travel to the Hole (node 0.)
 * Then, the game can end in three ways:
 *
 * 	If ever the Cat occupies the same node as the Mouse, the Cat wins.
 * 	If ever the Mouse reaches the Hole, the Mouse wins.
 * 	If ever a position is repeated (i.e., the players are in the same position as a previous turn, and it is the same player's turn to move), the game is a draw.
 *
 * Given a graph, and assuming both players play optimally, return
 *
 * 	1 if the mouse wins the game,
 * 	2 if the cat wins the game, or
 * 	0 if the game is a draw.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat1.jpg" style="width: 300px; height: 300px;" />
 * Input: graph = [[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]]
 * Output: 0
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/cat2.jpg" style="width: 200px; height: 200px;" />
 * Input: graph = [[1,3],[0],[3],[0,2]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	3 <= graph.length <= 50
 * 	1 <= graph[i].length < graph.length
 * 	0 <= graph[i][j] < graph.length
 * 	graph[i][j] != i
 * 	graph[i] is unique.
 * 	The mouse and the cat can always move.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cat-and-mouse/
// discuss: https://leetcode.com/problems/cat-and-mouse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    const MOUSE_WIN: i8 = 1;
    const CAT_WIN: i8 = 2;
    const DRAW: i8 = 0;

    fn dfs(
        graph: &[Vec<i32>],
        mem: &mut [Vec<Vec<i8>>],
        mouse_pos: usize,
        cat_pos: usize,
        turn: usize,
        n: usize,
    ) -> i8 {
        if mouse_pos == cat_pos {
            mem[mouse_pos][cat_pos][turn] = Self::CAT_WIN;
            return Self::CAT_WIN;
        }

        if mouse_pos == 0 {
            mem[mouse_pos][cat_pos][turn] = Self::MOUSE_WIN;
            return Self::MOUSE_WIN;
        }

        if turn >= 3 * n + 4 {
            mem[mouse_pos][cat_pos][turn] = Self::DRAW;
            return Self::DRAW;
        }

        if mem[mouse_pos][cat_pos][turn] != -1 {
            return mem[mouse_pos][cat_pos][turn];
        }

        // mouse turn
        if (turn & 1) == 0 {
            let mut res = Self::CAT_WIN;
            for &next_pos in &graph[mouse_pos] {
                let next_res = Self::dfs(graph, mem, next_pos as usize, cat_pos, turn + 1, n);
                if next_res == Self::MOUSE_WIN {
                    mem[mouse_pos][cat_pos][turn] = Self::MOUSE_WIN;
                    return Self::MOUSE_WIN;
                }

                if next_res == Self::DRAW {
                    res = Self::DRAW;
                }
            }
            mem[mouse_pos][cat_pos][turn] = res;
            return res;
        } else {
            let mut res = Self::MOUSE_WIN;
            for &next_pos in &graph[cat_pos] {
                if next_pos != 0 {
                    let next_res = Self::dfs(graph, mem, mouse_pos, next_pos as usize, turn + 1, n);
                    if next_res == Self::CAT_WIN {
                        mem[mouse_pos][cat_pos][turn] = Self::CAT_WIN;
                        return Self::CAT_WIN;
                    }

                    if next_res == Self::DRAW {
                        res = Self::DRAW;
                    }
                }
            }
            mem[mouse_pos][cat_pos][turn] = res;
            return res;
        }
    }

    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut mem = vec![vec![vec![-1; 3 * n + 4 + 1]; n]; n];
        Self::dfs(&graph, &mut mem, 1, 2, 0, n) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_913() {
        assert_eq!(
            Solution::cat_mouse_game(vec![
                vec![2, 5],
                vec![3],
                vec![0, 4, 5],
                vec![1, 4, 5],
                vec![2, 3],
                vec![0, 2, 3]
            ]),
            0
        );
        assert_eq!(
            Solution::cat_mouse_game(vec![vec![1, 3], vec![0], vec![3], vec![0, 2]]),
            1
        );
    }
}
