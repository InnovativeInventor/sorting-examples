use std::io::BufRead;
use std::collections::LinkedList;

fn main() {
    let input = std::io::stdin();

    let mut sorter = InsertionSortVec::new(1000000);

    for line in input.lock().lines(){
        let num: i64= line.unwrap().parse().unwrap();
        sorter.insert(num);
    }

    for integer in &sorter.list {
        println!("{}", integer)
    }

}

trait Sort {
    fn insert(&mut self, input: i64);
    fn new(cap: usize) -> Self;
}

struct InsertionSortVec {
    list: Vec<i64>
}

impl Sort for InsertionSortVec {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }
    fn insert(&mut self, input: i64) {
        if self.list.is_empty() {
            self.list.push(input);
        } else {
            for count in 0..self.list.len() {
                if &input < &self.list[count] { // invariant holds
                    self.list.insert(count, input);
                    return
                }
            }
            self.list.push(input);
        }
    }
}
