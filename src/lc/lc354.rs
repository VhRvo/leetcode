struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        // Self::from_start_pareto_points(envelopes)
        // Self::wrong_from_start_height(envelopes)
        Self::from_start_height(envelopes)
    }

    fn from_start_height(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes
            .into_iter()
            .map(|e| (e[0], e[1]))
            .collect::<Vec<_>>();
        envelopes.sort_by(|lhs, rhs| {
            if lhs.0 == rhs.0 {
                lhs.1.cmp(&rhs.1).reverse()
            } else {
                lhs.0.cmp(&rhs.0)
            }
        });
        let mut monotone_stack = Vec::new();
        for envelop in envelopes.iter() {
            let height = envelop.1;

            let point = monotone_stack.partition_point(|&probe: &i32| probe < height);

            if point == monotone_stack.len() {
                monotone_stack.push(height);
            } else {
                monotone_stack[point] = monotone_stack[point].min(height);
            }
        }
        monotone_stack.len() as i32
    }

    fn wrong_from_start_height(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|lhs, rhs| {
            if lhs[0] == rhs[0] {
                lhs[1].cmp(&rhs[1])
            } else {
                lhs[0].cmp(&rhs[0])
            }
        });
        let mut monotone_stack = Vec::new();
        for envelop in envelopes.iter() {
            let width = envelop[0];
            let height = envelop[1];

            let point = monotone_stack.partition_point(|probe: &(i32, i32)| {
                println!("probing {:?} vs ({},{})", probe, width, height);
                probe.0 < width && probe.1 < height
            });

            if point == monotone_stack.len() {
                monotone_stack.push((width, height));
            } else {
                if height < monotone_stack[point].1 {
                    // Wrong: pareto-points may be discarded here
                    monotone_stack[point] = (width, height);
                }
            }
        }
        monotone_stack.len() as i32
    }

    fn from_start_pareto_frontier(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // For envelopes with the same width, we keep track of pareto-optimal points.
        // A point is pareto-optimal if no other point dominates it (has both smaller width and height).
        // The envelope with the smallest height for a given width is the best candidate,
        // as it maximizes the chance of being nested by future envelopes.
        fn insert_to_monotone_stack(frontiers: &mut Vec<(i32, i32)>, width: i32, height: i32) {
            if let Some(top) = frontiers.last() {
                // If top.1 > height, then top.0 must not equal width (due to sorting order)
                if top.1 > height {
                    frontiers.push((width, height));
                }
            }
        }
        envelopes.sort_by(|lhs, rhs| {
            if lhs[0] == rhs[0] {
                lhs[1].cmp(&rhs[1])
            } else {
                lhs[0].cmp(&rhs[0])
            }
        });
        let mut monotone_stack = Vec::new();
        for envelop in envelopes.iter() {
            let width = envelop[0];
            let height = envelop[1];

            let point = monotone_stack.partition_point(|pareto_points: &Vec<(i32, i32)>| {
                for &(w, h) in pareto_points.iter() {
                    // We need to find the point before the same width group
                    // if w < width && h < height {
                    if w != width && h < height {
                        return true;
                    }
                }
                false
            });

            if point == monotone_stack.len() {
                monotone_stack.push(vec![(width, height)]);
            } else {
                insert_to_monotone_stack(&mut monotone_stack[point], width, height);
            }
        }
        monotone_stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![30, 50], vec![12, 2], vec![3, 4], vec![12, 15]]),
            3
        );
    }
}
