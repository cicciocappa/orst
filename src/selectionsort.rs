use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord {
            for unsorted in 0..slice.len() {
                 
                let smallest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_,v)|v)
                .map(|(i,_)|unsorted+i)
                .expect("slice not empty");

                /*
                let mut smallest = unsorted;
                for i in (unsorted+1)..slice.len(){
                    if slice[i]<slice[smallest] {
                        smallest = i;
                    }
                }*/
                if smallest!=unsorted {
                    slice.swap(unsorted, smallest);
                }
            }
       
        }
}

#[test]
fn it_works(){
    let mut things = vec![4,3,5,2,1];
    SelectionSort.sort(&mut things);
    assert_eq!(things,&[1,2,3,4,5]);
}