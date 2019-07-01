pub struct Solution;

impl Solution {
  #[allow(dead_code)]
  pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {

    nums.sort();

    let mut result = nums[0] + nums[1] + nums[nums.len() - 1];

    for i in 0..nums.len() - 2 {
      let x = nums[i];

      let mut j = i + 1;
      let mut k = nums.len() - 1;

      while j < k {
        let y = nums[j];
        let z = nums[k];
        let sum = x + y + z;

        if (sum - target).abs() < (result - target).abs() {
          result = sum;
        }

        if sum < target {
          j += 1;
          while j < k && nums[j] == nums[j - 1] {
            j += 1;
          }
        } else {
          k -= 1;
          while j < k && nums[k] == nums[k + 1] {
            k -= 1;
          }
        }
      }
    }

    return result;
  }
}

#[cfg(test)]
mod tests {

  use super::Solution;

  #[test]
  fn it_works() {
    {
      let nums = vec![-1, 2, 1, -4];
      let target = 1;
      assert_eq!(2, Solution::three_sum_closest(nums, target));
    }

    {
      let nums = vec![1, 1, 1, 1];
      let target = -100;
      assert_eq!(3, Solution::three_sum_closest(nums, target));
    }
  }
}