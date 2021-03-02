use std::io::BufRead;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "max-sorts", about = "An example of several sorting algos.")]
struct Opt {
    #[structopt(short, long)]
    insertion: bool,

    #[structopt(short, long)]
    merge: bool,

    #[structopt(short, long)]
    quicksort: bool,

    #[structopt(short, long)]
    default: bool,
}

fn main() {
    let opt = Opt::from_args();
    let input = std::io::stdin();
    let cap = 1000000;

    if opt.insertion {
        println!("Running insertion sort");
        let mut sorter = InsertionSortVec::new(1000000);

        for line in input.lock().lines(){
            let num: i64= line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in &sorter.list {
            println!("{}", integer)
        }
    } else if opt.merge {
        println!("Running merge sort");
        let mut sorter = MergeSortVec::new(cap);

        for line in input.lock().lines() {
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in sorter.sort() {
            println!("{}", integer)
        }
    } else if opt.quicksort {
        println!("Running quick sort");
        let mut sorter = QuickSortVec::new(cap);

        for line in input.lock().lines() {
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in sorter.sort() {
            println!("{}", integer)
        }
    } else {
        println!("Running default sort");
        // Rust's default sort
        let mut sorter = Vec::<i64>::with_capacity(cap);

        for line in input.lock().lines() {
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.push(num);
        }

        sorter.sort();

        for integer in sorter {
            println!("{}", integer)
        }

    }
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
    if a.len() == 0 {
        return b
    } else if b.len() == 0 {
        return a
    }

    let mut merged = Vec::<i64>::new();

    let mut i = 0; // counter for a
    let mut j = 0; // counter for b

    while i < a.len() || j < b.len() {
        if j >= b.len() {
            merged.push(a[i]);
            i += 1;
        } else if i >= a.len() {
            merged.push(b[j]);
            j += 1;
        } else if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }

    // if i == a.len() && j != b.len() {
    //     merged.extend_from_slice(&b.split_off(j));
    // }

    // if j == b.len() && i != a.len() {
    //     merged.extend_from_slice(&a.split_off(i));
    // }

    merged
}

impl MergeSortVec {
    fn sort(&mut self) -> Vec<i64> {
        // Inefficient fold -- yay HOFs!
        // TODO: implement binary fold
        return self.list.iter().fold(vec![], |merged_list, other| merge(merged_list, other.to_vec()))
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

fn quicksort(list: Vec<i64>, low: usize, high: usize) -> Vec<i64> {
    if low < high {
        let (mut list, pivot_loc) = partition(list, low, high);
        list = quicksort(list, low, pivot_loc - 1);
        list = quicksort(list, pivot_loc + 1, high);
        list
    } else {
        list
    }
}

fn partition(mut list: Vec<i64>, low: usize, high: usize) -> (Vec<i64>, usize) {
    let pivot = list[high];
    let mut i = low;
    for j in low..high {
        if list[j] <= pivot {
            list.swap(i, j);
            i += 1
        }
    }
    list.swap(i, high);

    (list, i)
}

impl QuickSortVec {
    fn sort(self) -> Vec<i64> {
        println!("Len: {}", self.list.len() - 1);
        // let length = (self.list.len() - 1) % self.list.len();
        let length = self.list.len()-1;
        quicksort(self.list, 0, length)
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
