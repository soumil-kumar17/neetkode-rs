use crate::contains_duplicates::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let len = nums.len();
        let mut result = Vec::new();
        if len < 3 {
            return result;
        }

        nums.sort_unstable();
        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}
