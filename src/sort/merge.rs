fn _merge2<T:Ord>(elements: &mut Vec<T>, low, half, high) {
    let mut low_index = low;
    let mut high_index = half;
    let mut circular_buffer_start = half;
    let mut circular_buffer_size = 0;
    loop {
        let mut insert_index = low_index;
        while elements[low_index] <= elements[high_index] {
            if circular_buffer_size != 0 {
                elements.swap(low_index,insert_index); 
                circular_buffer_size -=1;
                circular_buffer_start+=1;
                if cicular_buffer_size == 0 {
                    low_index = insert_index;
                } else {
                    insert_index+=1;
                }
            }
            low_index +=1;
        }
        if low_index >= high_index {
            return;
        }
        if circular_buffer_size !=0 {
            elements.swap(low_index, high_index);
            circular_buffer_size+=1;

        } else {
            elements.swap(low_index, high_index);
            insert_index = low_index+1;
            low_index=high_index;
            high_index +=1;
            circular_buffer_size+=1;
        }

    }
}
fn _merge<T:Ord>(elements: &mut Vec<T>, low, high) {
    let half = low + (high - low) / 2;
    if half != low {
        _merge(elements, low, half -1);
        _merge(elements, half, high);
        _merge2(elements, low, half, high);
    }
}

pub fn merge<T:Ord>(elements: &mut Vec<T>) {
    let half= elements.len()/2;
    if half == 0 {
        return;
    }
    _merge(elements, 0,half-1);
    _merge(elements, half,elements.len()-1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn merge_zero() {
        let mut res: Vec<i32> = vec![];
        merge(&mut res);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn merge_one() {
        let mut res: Vec<i32> = vec![1];
        merge(&mut res);
        assert_eq!(res, vec![1]);
    }
    #[test]
    fn merge_two_ordered() {
        let mut res: Vec<i32> = vec![1, 2];
        merge(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn merge_two_unordered() {
        let mut res: Vec<i32> = vec![2, 1];
        merge(&mut res);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn merge_three_ordered() {
        let mut res: Vec<i32> = vec![1, 2, 3];
        merge(&mut res);
        assert_eq!(res, vec![1, 2, 3]);
    }
    #[test]
    fn merge_many_unordered() {
        let mut res: Vec<i32> = vec![0, 4, 67, 6, 7, 0, 1, 2, 3];
        merge(&mut res);
        assert_eq!(res, vec![0, 0, 1, 2, 3, 4, 6, 7, 67]);
    }
}
