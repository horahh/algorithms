use std::collections::VecDeque;

/* Merge  Example  for 2 arrays
12345 223789
1 2345 223789
12 345 223789
122 45 3 23789
1222 5 34 3789
12223 54 3789
122233 45 789
1222334 5 789
12223345 789
122233457 89
1222334578 9
12223345789

00000 123456789   011234
00000 123456789   00011234
00000000 456789 123   11234     queue 1   
000000001 56789 4 23   11234     queue 2 4
00000000111 789 4 23 56  234    queue 2 4 5
000000001112 89 4 7 3 56  234    queue 3 4 5 7

*/

pub struct SideMetadata {
    pub index: usize,
    pub size: usize,
}

impl SideMetadata {
    fn new<T>(input_index: usize, input_size: usize, _elements: &mut Vec<T> ) -> SideMetadata {
        SideMetadata {
            index : input_index,
            size : input_size,
        }
    }
}

pub struct MergeMetadata<T: Copy>{
    pub q: VecDeque<T>,
    pub left: SideMetadata,
    pub right: SideMetadata,
    pub insert_index: usize,
}

impl<T> MergeMetadata<T>
    where T: std::fmt::Debug + Copy,
{
    fn new( left_index: usize, right_index: usize, total_size: usize, elements: &mut Vec<T>) -> MergeMetadata<T>
    {
        MergeMetadata::<T>{
            q: VecDeque::<T>::new(),
            left: SideMetadata::new(left_index, right_index - left_index, elements),
            right: SideMetadata::new(right_index, total_size - (right_index - left_index), elements),
            insert_index : left_index,
        }
    }
    fn insert_left(self: &mut MergeMetadata<T> , elements: &mut Vec<T>) 
    {
        if self.q.is_empty() {
            self.left.index+=1;
            self.left.size -=1;
            self.insert_index +=1;
            return;
        }
        //elements.swap(self.insert_index, self.left.index);
        self.q.push_back(elements[self.insert_index]);
        elements[self.insert_index] = self.q.pop_front().unwrap();
        self.left.size -=1;
        self.insert_index += 1;

        println!("left index: {}", self.left.index);
        println!("insert index {}", self.insert_index);
        println!("{:?} {:?} ", elements, self.q);
    }
    fn insert_right(self: &mut MergeMetadata<T> , elements: &mut Vec<T>) 
    {
        let right_value = self.get_right_value(elements);
        self.q.push_back(right_value);
        self.right.index +=1;
        self.right.size -=1;
        self.insert_index +=1;
        println!("{:?} {:?} ", elements, self.q);
    }
    fn get_left_value(self: &mut MergeMetadata<T>, elements: &mut Vec<T>) -> T
    {
        if !self.q.is_empty() {
            elements[self.insert_index]
        }
        else
        {
            self.q[0]
        }
    }
    fn get_right_value(self: &mut MergeMetadata<T>, elements: &mut Vec<T>) -> T
    {
        elements[self.right.index]
    }
}

fn _get_left_elements<T: Ord>(elements: &mut Vec<T>, meta: &mut MergeMetadata<T>)
    where
    T: std::fmt::Debug + Copy,
{
    while meta.left.size != 0 && meta.right.size != 0  && meta.get_left_value(elements) <= meta.get_right_value(elements) { 
        meta.insert_left(elements);
    }
    while meta.right.size == 0 && meta.left.size != 0 {
        meta.insert_left(elements);
    }
}

fn _get_right_elements<T: Ord>(elements: &mut Vec<T>, meta: &mut MergeMetadata<T>)
    where
    T: std::fmt::Debug + Copy,
{
    while meta.left.size != 0 && meta.right.size != 0  && meta.get_left_value(elements) > meta.get_right_value(elements) {
        meta.insert_right(elements);
    }
    while meta.left.size == 0 && meta.right.size !=0 {
        meta.insert_right(elements);
    }
}

fn _merge2<T: Ord>(elements: &mut Vec<T>, low: usize, half: usize, high: usize)
where
    T: std::fmt::Debug + Copy,
{
    let mut meta = MergeMetadata::new(low, half, high-low+1, elements);
    while meta.left.size != 0 || meta.right.size != 0 {
         _get_left_elements(elements, &mut meta);
         _get_right_elements(elements, &mut meta);
    }
}

fn _merge<T: Ord>(elements: &mut Vec<T>, low: usize, high: usize)
where
    T: std::fmt::Debug + Copy,
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
        for index in low..high-1 {
            if elements[index] > elements[index+1] {
                for i in 0..high {
                    if i == low || i == high || i == index || i == half {
                        print!(" | ");
                        continue
                    }
                    print!("   ");
                }
                println!(" | ");
                for _ in 0..index {
                    print!("   ");
                }
                println!("ERROR");
                println!("merge2::start low: {} half {} high {}", low, half, high);
                assert!(false);
            }
        }
        println!("merge2::start low: {} half {} high {}", low, half, high);
        println!("merge2::finish");
        println!("");
    }
}

pub fn merge<T: Ord>(elements: &mut Vec<T>)
where
    T: std::fmt::Debug + Copy,
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
    #[test]
    fn merge_many_unordered4() {
        let mut res: Vec<i32> = vec![6,7,8,0,1,1,1,2,2,2,0,0,0,0,0,1,1,1,2,2,2,0,1,1,1,2,2,2,0,0,0,0,0,1,1,1,2,2,2,5,5];
        // let mut res: Vec<i32> = vec![67, 7, 6, 4, 3, 2, 1, 0, 0];
        merge(&mut res);
        assert_eq!(res, vec![0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,5,5,6,7,8]);
    }
}
