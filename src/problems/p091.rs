struct Solution {}

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        let mut i: usize = 1;
        let n = costs.len();
        while i < n {
            costs[i][0] += costs[i - 1][1].min(costs[i - 1][2]);
            costs[i][1] += costs[i - 1][0].min(costs[i - 1][2]);
            costs[i][2] += costs[i - 1][0].min(costs[i - 1][1]);
            i += 1;
        }

        *costs[n - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_091() {
        assert_eq!(
            Solution::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
            10
        );
        assert_eq!(Solution::min_cost(vec![vec![7, 6, 2]]), 2);
    }
}
