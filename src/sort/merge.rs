fn _get_next(
    low_index: usize,
    insert_index: usize,
    circular_buffer_size: usize,
    buffer_partition: usize,
) -> usize {
    let mut offset;
    if low_index == insert_index {
        return low_index + 1;
    } else {
        println!("low {} insert {}/n", low_index, insert_index);
        offset = (low_index + 1 - insert_index) % circular_buffer_size;
    }

    // this is the complex case where the buffer has three segments
    if low_index > buffer_partition && insert_index < buffer_partition {
        //  not necessary to check for condition 2 && offset < (low_index - insert_index)
        let nested_buffer_size = buffer_partition - low_index;
        if offset > nested_buffer_size {
            // subtract the size of the nested buffer
            offset -= nested_buffer_size;
        } else {
            // move offset to the nester buffer region
            offset += buffer_partition - insert_index;
        }
    }
    return low_index + offset;
}

fn _rotate_circular_buffer<T: Ord>(
    elements: &mut Vec<T>,
    mut low_index: usize,
    mut insert_index: usize,
    mut circular_buffer_size: usize,
    buffer_partition: usize,
) {
    while circular_buffer_size != 0 {
        elements.swap(low_index, insert_index);
        low_index = _get_next(
            low_index,
            insert_index,
            circular_buffer_size,
            buffer_partition,
        ); // this should just add 1?
        circular_buffer_size -= 1;
        insert_index += 1;
    }
}

fn _merge2<T: Ord>(elements: &mut Vec<T>, low: usize, half: usize, high: usize)
where
    T: std::fmt::Debug,
{
    let mut low_index = low;
    let mut high_index = half;
    let mut circular_buffer_size = half - low;
    let mut insert_index = low_index;
    loop {
        while elements[low_index] <= elements[high_index] {
            println!("test low index {} high index {}", low_index, high_index);
            if circular_buffer_size == 0 {
                return;
            }
            println!("circular buffer size: {}", circular_buffer_size);
            println!("swap low index {} insert index {}", low_index, insert_index);
            println!("before  {:?} ", elements);
            elements.swap(low_index, insert_index);
            println!("after   {:?} ", elements);
            low_index = _get_next(low_index, insert_index, circular_buffer_size, half);
            circular_buffer_size -= 1;
            insert_index += 1;
        }
        if circular_buffer_size == 0 {
            return;
        }
        println!(
            "swap insert index {} high index {}",
            insert_index, high_index
        );
        println!("before  {:?} ", elements);
        elements.swap(insert_index, high_index);
        println!("after   {:?} ", elements);
        if low_index != insert_index {
            low_index = high_index;
        }
        insert_index += 1;
        high_index += 1;
        if high_index == high {
            _rotate_circular_buffer(
                elements,
                low_index,
                insert_index,
                circular_buffer_size,
                half,
            );
            return;
        }
    }
}
fn _merge<T: Ord>(elements: &mut Vec<T>, low: usize, high: usize)
where
    T: std::fmt::Debug,
{
    let half = low + (high - low + 1) / 2;
    if half != low {
        println!("{:?} ", elements);
        println!(" merge:: low {:?} high {:?}", low, half);
        _merge(elements, low, half );
        println!(" merge:: low {:?} high {:?}", half+1, high);
        _merge(elements, half+1, high);
        println!("merge2::");
        _merge2(elements, low, half, high);
    }
}

pub fn merge<T: Ord>(elements: &mut Vec<T>)
where
    T: std::fmt::Debug,
{
    let half = elements.len() / 2;
    if half == 0 {
        return;
    }
    _merge(elements, 0, half );
    _merge(elements, half+1, elements.len() - 1);
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
        // let mut res: Vec<i32> = vec![67, 7, 6, 4, 3, 2, 1, 0, 0];
        merge(&mut res);
        assert_eq!(res, vec![0, 0, 1, 2, 3, 4, 6, 7, 67]);
    }
}
