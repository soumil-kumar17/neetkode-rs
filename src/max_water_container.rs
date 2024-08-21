use std::cmp;

use crate::contains_duplicates::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;

        while left < right {
            let width = (right - left) as i32;
            let h = cmp::min(height[left], height[right]);
            res = cmp::max(res, h * width);

            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        res
    }
}
