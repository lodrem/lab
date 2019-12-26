pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut res = vec![];

        let mut s = VecDeque::default();

        for asteroid in asteroids.iter() {
            if *asteroid > 0 {
                s.push_back(*asteroid);
            } else {
                let mut exploded = *asteroid;

                while let Some(right_asteroid) = s.pop_back() {
                    exploded = *asteroid + right_asteroid;

                    if exploded > 0 {
                        s.push_back(right_asteroid);
                        break;
                    } else if exploded == 0 {
                        break;
                    }
                }

                if exploded < 0 {
                    res.push(*asteroid);
                }
            }
        }

        res.extend(s);

        res
    }
}
