fn _get_next(
    low_index: usize,
    insert_index: usize,
    circular_buffer_size: usize,
    buffer_partition: usize,
) -> usize {
    if low_index == insert_index {
        println!("first case low {} insert {}", low_index+1, insert_index);
        return low_index + 1;
    } 
    // this case swap the next min with the current min
    if low_index == insert_index + circular_buffer_size - 1 {
        println!("second case low {} insert {}", low_index, insert_index);
        return low_index;
    }
    if low_index+1 - insert_index < circular_buffer_size {
        println!("third case low {} insert {}", low_index+1, insert_index);
        return low_index + 1 ;
    }
    // this is the complex case where the buffer has three segments
    if insert_index < buffer_partition {
        println!("4th case low {} insert {}", buffer_partition, insert_index);
        return buffer_partition;
    }
    println!("error");
    //println!(
    return  insert_index+1;
}

fn _rotate_circular_buffer<T: Ord>(
    elements: &mut Vec<T>,
    mut low_index: usize,
    mut insert_index: usize,
    mut circular_buffer_size: usize,
    buffer_partition: usize,
)
where
    T: std::fmt::Debug,
{
    if insert_index == low_index {
        return;
    }
    println!("buffer size: {}", circular_buffer_size);
    while circular_buffer_size != 0 {
    println!("buffer size: {}", circular_buffer_size);
        elements.swap(low_index, insert_index);
        low_index = _get_next(
            low_index,
            insert_index,
            circular_buffer_size,
            buffer_partition,
        ); // this should just add 1?
        circular_buffer_size -= 1;
        insert_index += 1;
        println!("{:?}", elements);
    }
}

fn _merge2<T: Ord>(elements: &mut Vec<T>, low: usize, mut half: usize, high: usize)
where
    T: std::fmt::Debug,
{
    let mut low_index = low;
    let mut high_index = half;
    let mut circular_buffer_size = half - low;
    let mut insert_index = low_index;
    loop {
        println!("circular buffer size {} low index {} high index {}", circular_buffer_size, low_index, high_index);
        while circular_buffer_size != 0 && elements[low_index] <= elements[high_index] {
            println!("test low index {} high index {}", low_index, high_index);
            println!("circular buffer size: {}", circular_buffer_size);
            println!("swap low index {} insert index {}", low_index, insert_index);
            println!("before1  {:?} ", elements);
            elements.swap(low_index, insert_index);
            println!("after1   {:?} ", elements);
            low_index = _get_next(low_index, insert_index, circular_buffer_size, half);
            println!("low :: {}", low_index);
            circular_buffer_size -= 1;
            insert_index += 1;
            if insert_index == half - 1 {
                half = low_index;
            }
        }
        if circular_buffer_size == 0 {
            return;
        }
        println!(
            "swap insert index {} high index {}",
            insert_index, high_index
        );
        println!("before2  {:?} ", elements);
        elements.swap(insert_index, high_index);
        println!("after2   {:?} ", elements);
        // move low to the swappped value index which was the high
        if low_index == insert_index  {
            low_index = high_index;
        }
        insert_index += 1;
        if insert_index == half -1 {
            half = low_index;
        }
        high_index += 1;
        println!("high index : {} high {} " , high_index, high);
        if high_index > high {
            _rotate_circular_buffer(
                elements,
                low_index,
                insert_index,
                circular_buffer_size,
                half,
            );
            println!("\n\nEXIT\n");
            return;
        }
    }
}
fn _merge<T: Ord>(elements: &mut Vec<T>, low: usize, high: usize)
where
    T: std::fmt::Debug,
{
    let half = low + (high - low + 1) / 2;
    println!("low {} high {}", low, high);
    if half != low {
        println!("{:?} ", elements);
        println!(" merge:: low {:?} high {:?}", low, half-1);
        _merge(elements, low, half-1 );
        println!(" merge:: low {:?} high {:?}", half, high);
        _merge(elements, half, high);
        println!("merge2::start low: {} half {} high {}", low, half, high);
        _merge2(elements, low, half, high);
        println!("{:?}", elements);
    println!("merge2::finish");
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
    _merge(elements, 0, elements.len()-1 );
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
    #[test]
    fn merge_many_unordered2() {
        let mut res: Vec<i32> = vec![0, 4, 67, 6, 7, 0, 1, 2, 3, 67, 7, 6, 4, 3, 2, 1, 0, 0];
        // let mut res: Vec<i32> = vec![67, 7, 6, 4, 3, 2, 1, 0, 0];
        merge(&mut res);
        assert_eq!(res, vec![0, 0, 0,0,1,1, 2,2, 3,3, 4,4, 6,6, 7,7, 67,67]);
    }
    #[test]
    fn merge_many_unordered3() {
        let mut res: Vec<i32> = vec![20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];
        // let mut res: Vec<i32> = vec![67, 7, 6, 4, 3, 2, 1, 0, 0];
        merge(&mut res);
        assert_eq!(res, vec![0, 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
    }
}
