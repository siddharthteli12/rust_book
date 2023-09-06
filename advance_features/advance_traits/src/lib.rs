trait IteratorAdd<Rhs = Self> {
    type Output;
    fn iter_both(self, rhs: Rhs) -> Option<Self::Output>;
}

impl IteratorAdd<Vec<f64>> for Vec<i32> {
    type Output = Vec<f64>;
    fn iter_both(self, rhs: Vec<f64>) -> Option<Self::Output> {
        let mut output: Vec<f64>;
        if self.len() > rhs.len() {
            output = self.iter().map(|item| *item as f64).collect();
            rhs.iter().enumerate().for_each(|(index, item)| {
                output[index] += item;
            });
        } else {
            output = rhs;
            self.iter()
                .map(|item| *item as f64)
                .enumerate()
                .for_each(|(index, item)| {
                    output[index] += item;
                })
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_simple_list() {
        let list1 = vec![1, 2, 3, 4, 5];
        let list2 = vec![1.2, 2.5, 2.7, 8.9];
        assert_eq!(list1.iter_both(list2), Some(vec![2.2, 4.5, 5.7, 12.9, 5.0]));
    }

    #[test]
    fn with_simple_list_2() {
        let list1 = vec![10, 12, 13];
        let list2 = vec![9.7, 4.54, 45.45, 343.0, 23.0, 12.0];
        assert_eq!(
            list1.iter_both(list2),
            Some(vec![19.7, 16.54, 58.45, 343.0, 23.0, 12.0])
        );
    }
}
