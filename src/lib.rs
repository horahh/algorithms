/// Brief.
/// return the index to the pivot.
/// Description
/// Takes the first value and moves it around be in the right place to be the pivot
///  * `elements` - The vector containing the elements to order.
///  * `start` - The index of the start of container segment
///  * `end` - The index of the end of container segment
fn _get_pivot<T: Ord>(elements: &mut Vec<T>, start: usize, end: usize) -> usize {
    let mut pivot_index = start;
    for lower_element_index in start + 1..end + 1 {
        if elements[pivot_index] > elements[lower_element_index] {
            elements.swap(pivot_index, lower_element_index);
            // swap the number that is less than the pivot with the number greater than pivot
            if pivot_index + 1 < lower_element_index {
                elements.swap(pivot_index + 1, lower_element_index)
            }
            // advance pivot once everything is coherent
            pivot_index += 1;
        }
    }
    pivot_index
}
pub fn _qsort<T: Ord>(elements: &mut Vec<T>, begin: usize, end: usize) {
    // three or more segment
    let pivot_index = _get_pivot(elements, begin, end);
    // qsort the rigth of the pivot
    if pivot_index != end {
        _qsort(elements, pivot_index + 1, end);
    }
    // qsort the left of the pivot
    if pivot_index != begin {
        _qsort(elements, begin, pivot_index - 1);
    }
}
pub fn qsort<T: Ord>(elements: &mut Vec<T>) {
    if elements.len() <= 1 {
        return;
    }
    let len: usize = elements.len();
    _qsort(elements, 0, len - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn qsort_zero() {
        let mut res: Vec<i32> = vec![];
        qsort(&mut res);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn qsort_one() {
        let mut res: Vec<i32> = vec![1];
        qsort(&mut res);
        assert_eq!(res, vec![1]);
    }
    #[test]
    fn qsort_two_ordered() {
        let mut res: Vec<i32> = vec![1, 2];
        qsort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn qsort_two_unordered() {
        let mut res: Vec<i32> = vec![2, 1];
        qsort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn qsort_three_ordered() {
        let mut res: Vec<i32> = vec![1, 2, 3];
        qsort(&mut res);
        assert_eq!(res, vec![1, 2, 3]);
    }
    #[test]
    fn qsort_many_unordered() {
        let mut res: Vec<i32> = vec![0, 4, 67, 6, 7, 0, 1, 2, 3];
        qsort(&mut res);
        assert_eq!(res, vec![0, 0, 1, 2, 3, 4, 6, 7, 67]);
    }
}
