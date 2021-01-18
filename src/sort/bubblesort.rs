pub fn bubblesort<T: Ord>(elements: &mut Vec<T>)
where
    T: std::fmt::Debug,
{
    if elements.len() == 0 {
        return;
    }
    for min_index in 0..elements.len() - 1 {
        let mut swap_index = min_index;
        for compare_index in min_index + 1..elements.len() {
            if elements[swap_index] > elements[compare_index] {
                swap_index = compare_index;
            }
        }
        elements.swap(min_index, swap_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn bubblesort_zero() {
        let mut res: Vec<i32> = vec![];
        bubblesort(&mut res);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn bubblesort_one() {
        let mut res: Vec<i32> = vec![1];
        bubblesort(&mut res);
        assert_eq!(res, vec![1]);
    }
    #[test]
    fn bubblesort_two_ordered() {
        let mut res: Vec<i32> = vec![1, 2];
        bubblesort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn bubblesort_two_unordered() {
        let mut res: Vec<i32> = vec![2, 1];
        bubblesort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn bubblesort_three_ordered() {
        let mut res: Vec<i32> = vec![1, 2, 3];
        bubblesort(&mut res);
        assert_eq!(res, vec![1, 2, 3]);
    }
    #[test]
    fn bubblesort_many_unordered() {
        let mut res: Vec<i32> = vec![0, 4, 67, 6, 7, 0, 1, 2, 3];
        bubblesort(&mut res);
        assert_eq!(res, vec![0, 0, 1, 2, 3, 4, 6, 7, 67]);
    }
}
