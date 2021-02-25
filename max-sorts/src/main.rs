use std::io::BufRead;

fn main() {
    let input = std::io::stdin();
    let cap = 1000000;

    // Insertion Sort
    // let mut sorter = InsertionSortVec::new(1000000);

    // for line in input.lock().lines(){
    //     let num: i64= line.unwrap().parse().unwrap();
    //     sorter.insert(num);
    // }

    // for integer in &sorter.list {
    //     println!("{}", integer)
    // }

    // Merge Sort
    // let mut sorter = MergeSortVec::new(cap);

    // for line in input.lock().lines() {
    //     let num: i64 = line.unwrap().parse().unwrap();
    //     sorter.insert(num);
    // }

    // let list = sorter.sort();

    // for integer in list {
    //     println!("{}", integer)
    // }

    // Merge Sort
    let mut sorter = QuickSortVec::new(cap);

    for line in input.lock().lines() {
        let num: i64 = line.unwrap().parse().unwrap();
        sorter.insert(num);
    }

    let list = sorter.sort();

    for integer in list {
        println!("{}", integer)
    }

    // Rust's default sort
    // let mut sorter = Vec::<i64>::with_capacity(cap);

    // for line in input.lock().lines() {
    //     let num: i64 = line.unwrap().parse().unwrap();
    //     sorter.push(num);
    // }

    // sorter.sort();

    // for integer in sorter {
    //     println!("{}", integer)
    // }
}

trait Sort {
    fn insert(&mut self, input: i64);
    fn new(cap: usize) -> Self;
}

struct InsertionSortVec {
    list: Vec<i64>,
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
                if &input < &self.list[count] {
                    // invariant holds
                    self.list.insert(count, input);
                    return;
                }
            }
            self.list.push(input);
        }
    }
}

struct MergeSortVec {
    list: Vec<Vec<i64>>,
}

fn merge(mut a: Vec<i64>, mut b: Vec<i64>) -> Vec<i64> {
    let mut merged = Vec::<i64>::new();

    let mut i = 0; // counter for a
    let mut j = 0; // counter for b

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(a[j]);
            j += 1;
        }
    }

    if i == a.len() && j != b.len() {
        merged.extend_from_slice(&b.split_off(j));
    }

    if j == b.len() && i != a.len() {
        merged.extend_from_slice(&a.split_off(i));
    }

    merged
}

impl MergeSortVec {
    fn sort(&mut self) -> Vec<i64> {
        if self.list.is_empty() {
            return vec![];
        }

        let merged_list: Vec<i64> = vec![];

        while merged_list.len() != 1 {
            let mut merged_list: Vec<i64> = vec![];
            while self.list.is_empty() {
                if self.list.len() == 1 {
                    merged_list.append(&mut self.list[0])
                } else {
                    merged_list.append(&mut merge(
                        self.list.pop().unwrap(),
                        self.list.pop().unwrap(),
                    ));
                }
            }

            self.list.append(&mut vec![merged_list]);
        }

        merged_list
    }
}

impl Sort for MergeSortVec {
    fn new(_cap: usize) -> Self {
        Self {
            // list: Vec::<Vec<i64>>::with_cap(cap),
            list: Vec::<Vec<i64>>::new(),
        }
    }
    fn insert(&mut self, input: i64) {
        if self.list.is_empty() {
            self.list.push(vec![input]);
        } else {
            let section = self.list.last_mut().unwrap();

            if section.len() == 2 || section[0] <= input {
                // push to back
                self.list.push(vec![input]);
            } else {
                section.insert(0, input);
            }
        }
    }
}

struct QuickSortVec {
    list: Vec<i64>,
}

fn quicksort(mut list: Vec<i64>) -> Vec<i64> {
    if list.len() <= 1 {
        return list
    } else {
        let size = list.len();

        // we choose a random pivot, since the input is sorted
        let pivot = list[size];
        let mut start = 0;
        for i in 0..size {
            if list[i] < pivot {
                list.swap(start, i);
                start += 1;
            }
        }
        list.swap(start, size);

        let mut combined_result = quicksort(list.split_off(start));
        combined_result.append(&mut quicksort(list));
        combined_result
    }

}

impl QuickSortVec {
    fn sort(self) -> Vec<i64> {
        quicksort(self.list)
    }
}

impl Sort for QuickSortVec {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }

    fn insert(&mut self, input: i64) {
        self.list.push(input);
    }


}
