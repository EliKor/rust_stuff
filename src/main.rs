use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianList {
    pub median: Option<u32>,
    min_heap: BinaryHeap<Reverse<u32>>,
    max_heap: BinaryHeap<u32>,
}

impl MedianList {
    pub fn new() -> Self {
        MedianList {
            median: None,
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }
    
    pub fn insert(&mut self, x: u32) {
        match (self.median, self.min_heap.peek(), self.max_heap.peek()) {
            (None, None, None) => { 
                self.median = Some(x);
            }
            (None, Some(Reverse(greater)), Some(lesser)) => {
                if *lesser <= x && x <= *greater {
                    self.median = Some(x);
                }
                else if x <= *lesser {
                    self.median = Some(*lesser);
                    self.max_heap.pop();
                    self.max_heap.push(x);
                }
                else {
                    self.median = Some(*greater);
                    self.min_heap.pop();
                    self.min_heap.push(Reverse(x));
                }
            }
            (Some(old_median), _, _) => { 
                let greater = { if x > old_median { x } else { old_median } };
                let lesser = { if x > old_median { old_median } else { x } };
                self.median = None;
                self.min_heap.push(Reverse(greater));
                self.max_heap.push(lesser);
            }
            _ => panic!("Median list is not in a valid state")
        }
    }

    pub fn get_median(&self) -> Option<f32> {
        match (self.median, self.min_heap.peek(), self.max_heap.peek()) {
            (Some(median), _, _) => {
                Some(median as f32)
            }
            (_, Some(Reverse(greater)), Some(lesser)) => {
                Some((*greater as f32 + *lesser as f32) / 2.0)
            }
            _ => None
        }
    }

}


#[cfg(test)]
mod test {
    use super::MedianList;
    #[test]
    fn general_test() {
        let mut ml = MedianList::new();
        assert_eq!(ml.get_median(), None);
        ml.insert(1);
        assert_eq!(ml.get_median(), Some(1.0));
        ml.insert(3);
        assert_eq!(ml.get_median(), Some(2.0));
        ml.insert(2);
        assert_eq!(ml.get_median(), Some(2.0));
        ml.insert(6);
        assert_eq!(ml.get_median(), Some(2.5));
        ml.insert(4);
        assert_eq!(ml.get_median(), Some(3.0));
        ml.insert(7);
        assert_eq!(ml.get_median(), Some(3.5));
        ml.insert(8);
        assert_eq!(ml.get_median(), Some(4.0));
        ml.insert(5);
        assert_eq!(ml.get_median(), Some(4.5));
    }
}
    

fn main() {
    println!("Nothing in main right now :(");
}
