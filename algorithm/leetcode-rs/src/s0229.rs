pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        // The Boyer-Moore Majority Vote Algorithm
        // https://gregable.com/2013/10/majority-vote-algorithm-find-majority.html
        // http://www.cs.rug.nl/~wim/pub/whh348.pdf

        // threshold = n / 3
        // candidate1 => first (> n / 3)
        // candidate2 => second (> n / 3)
        // no more candidate3 because (1 - >(n / 3) + >(n / 3)) < n / 3

        let mut count1 = 0;
        let mut candidate1 = 0;
        let mut count2 = 0;
        let mut candidate2 = 0;

        for num in nums.iter() {
            match *num {
                n if n == candidate1 => count1 += 1,
                n if n == candidate2 => count2 += 1,
                _ => {
                    if count1 == 0 {
                        candidate1 = *num;
                        count1 += 1;
                    } else if count2 == 0 {
                        candidate2 = *num;
                        count2 += 1;
                    } else {
                        count1 -= 1;
                        count2 -= 1;
                    }
                }
            }
        }

        let mut res = vec![];

        let threshold = nums.len() / 3;
        let mut count1 = 0;
        let mut count2 = 0;

        for num in nums.into_iter() {
            match num {
                n if n == candidate1 => count1 += 1,
                n if n == candidate2 => count2 += 1,
                _ => {}
            }
        }

        if count1 > threshold {
            res.push(candidate1);
        }

        if count2 > threshold {
            res.push(candidate2);
        }

        res
    }
}
