use super::binary_search::BinarySearch;

pub trait LIS<T> {
    fn lis_weakly(&self) -> Vec<T>;
    fn lis_strictly(&self) -> Vec<T>;
}

impl<T> LIS<T> for [T]
where
    T: Clone + Ord,
{
    fn lis_weakly(&self) -> Vec<T> {
        let mut dp = vec![];
        let mut prev = vec![None; self.len()];
        for i in 0..self.len() {
            let j = dp.upper_bound(self[i].clone());
            if j > 0 {
                prev[i] = Some(dp[j - 1].clone());
            }
            if dp.len() == j {
                dp.push(self[i].clone());
            } else {
                dp[j] = self[i].clone()
            }
        }

        if let Some(temp) = dp.last() {
            let mut look_for = temp.clone();
            let mut res = vec![];
            for i in (0..self.len()).rev() {
                if self[i] == look_for {
                    res.push(look_for);
                    if let Some(next) = prev[i].clone() {
                        look_for = next;
                    } else {
                        break;
                    }
                }
            }
            res.reverse();
            res
        } else {
            vec![]
        }
    }

    fn lis_strictly(&self) -> Vec<T> {
        let mut dp = vec![];
        let mut prev = vec![None; self.len()];
        for i in 0..self.len() {
            let j = dp.lower_bound(self[i].clone());
            if j > 0 {
                prev[i] = Some(dp[j - 1].clone());
            }
            if dp.len() == j {
                dp.push(self[i].clone());
            } else {
                dp[j] = self[i].clone()
            }
        }

        if let Some(temp) = dp.last() {
            let mut look_for = temp.clone();
            let mut res = vec![];
            for i in (0..self.len()).rev() {
                if self[i] == look_for {
                    res.push(look_for);
                    if let Some(next) = prev[i].clone() {
                        look_for = next;
                    } else {
                        break;
                    }
                }
            }
            res.reverse();
            res
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_increasing_subsequence::LIS;

    #[test]
    fn it_works() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

        assert_eq!(v.lis_strictly(), vec![1, 2, 3, 5, 7, 9]);
        assert_eq!(v.lis_weakly(), vec![1, 1, 2, 3, 5, 8, 9, 9]);
    }
}
