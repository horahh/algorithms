//! Crate for an educative an clean example of the quicksort algorithm
//!
//! Avoided use of casting and using debug on generic types for clarity.
//! To show the output when running the tests simply execute:
//! cargo test -- --nocapture

/// Brief.
/// return the index to the pivot.
/// Description
/// Takes the first value and moves it around be in the right place to be the pivot
///  * `elements` - The vector containing the elements to order.
///  * `start` - The index of the start of container segment
///  * `end` - The index of the end of container segment
fn _get_pivot<T: Ord>(elements: &mut Vec<T>, start: usize, end: usize) -> usize
where
    T: std::fmt::Debug,
{
    let mut pivot_index = start;
    let mut last_greater_index = end + 1;
    'lower_loop: for lower_index in start + 1..end + 1 {
        // avoid extra loops when knowing the right part has greater numbers
        if lower_index == last_greater_index {
            break;
        }
        if elements[lower_index] > elements[pivot_index] {
            for greater_index in (lower_index + 1..last_greater_index).rev() {
                if elements[greater_index] < elements[pivot_index] {
                    println!(" swap: pivot_value {:?} pivot_index {} lower_value {:?} lower_index{} greater_value {:?} greater_index {} " , elements[pivot_index] , pivot_index, elements[lower_index]  , lower_index, elements[greater_index] , greater_index);
                    elements.swap(lower_index, greater_index);
                    println!(" {:?}", elements);
                    last_greater_index = greater_index;
                    continue 'lower_loop;
                }
            }
            // Reaching this means the greater and lower indexes are ordered
            // Then swap pivot with the greatest lower index to have the right partitions
            println!(
                " swap: pivot_value {:?} pivot_index {} lower_value {:?} lower_index {} ",
                elements[pivot_index],
                pivot_index,
                elements[lower_index - 1],
                lower_index - 1
            );
            println!(" {:?}", elements);
            elements.swap(pivot_index, lower_index - 1);
            pivot_index = lower_index - 1;
            return pivot_index;
        }
    }
    // when not exiting inside the loop means all the elements are lower than pivot
    elements.swap(pivot_index, last_greater_index - 1);
    println!(" after3 {:?}", elements);
    pivot_index = end;
    pivot_index
}
pub fn _qsort<T: Ord>(elements: &mut Vec<T>, begin: usize, end: usize)
where
    T: std::fmt::Debug,
{
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

/// Implementation of quicksort algorithm
/// * `elements` - A vector of any size containing a generic type that can be ordered.
pub fn qsort<T: Ord>(elements: &mut Vec<T>)
where
    T: std::fmt::Debug,
{
    if elements.len() <= 1 {
        return;
    }
    let len: usize = elements.len();
    println!(" {:?}", elements);
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
