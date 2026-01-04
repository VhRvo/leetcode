struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let target = target as i64;
        nums.sort();
        let mut result = Vec::new();

        let last_three_sum = if nums.len() >= 4 {
            nums[nums.len() - 3] as i64 + nums[nums.len() - 2] as i64 + nums[nums.len() - 1] as i64
        } else {
            return result;
        };
        for ii in 0..nums.len() - 3 {
            let ii_num = nums[ii];
            if ii > 0 && nums[ii - 1] == ii_num {
                continue;
            }
            if ii_num as i64 + nums[ii + 1] as i64 + nums[ii + 2] as i64 + nums[ii + 3] as i64
                > target
            {
                break;
            }
            if ii_num as i64 + last_three_sum < target {
                continue;
            }
            for jj in ii + 1..nums.len() - 2 {
                let jj_num = nums[jj];
                if jj > ii + 1 && nums[jj - 1] == jj_num {
                    continue;
                }
                let mut kk = jj + 1;
                let mut ee = nums.len() - 1;
                while kk < ee {
                    let kk_num = nums[kk];
                    let ee_num = nums[ee];
                    if kk > jj + 1 && nums[kk - 1] == kk_num {
                        kk += 1;
                        continue;
                    }
                    if ee < nums.len() - 1 && ee_num == nums[ee + 1] {
                        ee -= 1;
                        continue;
                    }
                    let sum = ii_num as i64 + nums[jj] as i64 + nums[kk] as i64 + nums[ee] as i64;
                    match sum.cmp(&target) {
                        Ordering::Less => {
                            kk += 1;
                        }
                        Ordering::Equal => {
                            result.push(vec![nums[ii], nums[jj], nums[kk], nums[ee]]);
                            kk += 1;
                            ee -= 1;
                        }
                        Ordering::Greater => {
                            ee -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}
