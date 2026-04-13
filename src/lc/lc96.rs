struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for total in 1..=n {
            // 动作：选 root 作为根
            for root in 1..=total {
                // 左子树节点数
                let left = root - 1;
                // 右子树节点数
                let right = total - root;
                dp[total] += dp[left] * dp[right];
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
}
