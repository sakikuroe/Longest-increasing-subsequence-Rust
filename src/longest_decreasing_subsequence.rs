use crate::longest_increasing_subsequence::LIS;

pub trait LDS<T> {
    fn lds_weakly(&self) -> Vec<T>;
    fn lds_strictly(&self) -> Vec<T>;
}

impl<T> LDS<T> for [T]
where
    T: Clone + Ord,
{
    fn lds_weakly(&self) -> Vec<T> {
        let rev_self = {
            let mut temp = self.clone().to_vec();
            temp.reverse();
            temp
        };

        let res = {
            let mut temp = rev_self.lis_weakly();
            temp.reverse();
            temp
        };
        res
    }

    fn lds_strictly(&self) -> Vec<T> {
        let rev_self = {
            let mut temp = self.clone().to_vec();
            temp.reverse();
            temp
        };

        let res = {
            let mut temp = rev_self.lis_strictly();
            temp.reverse();
            temp
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_decreasing_subsequence::LDS;

    #[test]
    fn it_works() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

        assert_eq!(v.lds_strictly(), vec![9, 6, 5, 3]);
        assert_eq!(v.lds_weakly(), vec![9, 6, 5, 3, 3]);
    }
}
