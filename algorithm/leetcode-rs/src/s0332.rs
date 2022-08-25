pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        use std::collections::HashMap;

        let mut targets = HashMap::new();

        for mut ticket in tickets.into_iter() {
            let departure = std::mem::replace(&mut ticket[0], Default::default());
            let arrival = std::mem::replace(&mut ticket[1], Default::default());

            let target = targets.entry(departure).or_insert(vec![]);
            target.push(arrival);
            target.sort_by(|x, y| y.cmp(&x));
        }

        let mut route = vec![];
        let mut stack = vec!["JFK".to_owned()];

        while !stack.is_empty() {
            while let Some(arrivals) = targets.get_mut(&stack[stack.len() - 1]) {
                if arrivals.is_empty() {
                    break;
                }

                stack.push(arrivals.pop().unwrap());
            }
            route.push(stack.pop().unwrap());
        }

        route.reverse();
        route
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        {
            let expected: Vec<String> = vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect();
            assert_eq!(
                Solution::find_itinerary(vec![
                    vec!["MUC", "LHR"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["JFK", "MUC"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["SFO", "SJC"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["LHR", "SFO"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect()
                ]),
                expected
            );
        }

        {
            let expected: Vec<String> = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect();
            assert_eq!(
                Solution::find_itinerary(vec![
                    vec!["JFK", "SFO"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["JFK", "ATL"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["SFO", "ATL"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["ATL", "JFK"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(),
                    vec!["ATL", "SFO"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect()
                ]),
                expected
            );
        }
    }
}
