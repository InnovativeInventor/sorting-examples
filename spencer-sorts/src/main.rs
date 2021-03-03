use std::io::BufRead;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "spencer-sorts", about = "A potpourri of sorting algorithms")]
struct Opt {
    #[structopt(short, long)]
    insertion: bool,

    #[structopt(short, long)]
    quick: bool,
}

fn main() {
    let opt = Opt::from_args();
    let input = std::io::stdin();
    let cap = 1000000;

    if opt.insertion {
        let mut sorter = InsertionSortVector::new(cap);

        for line in input.lock().lines(){
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in &sorter.list {
            println!("{}", integer);
        }
    } else if opt.quick {
        let mut sorter = QuickSortVector::new(cap);

        for line in input.lock().lines(){
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in sorter.sort() {
            println!("{}", integer);
        }
    }
}

trait Sortable {
    fn new(cap: usize) -> Self;
    fn insert(&mut self, input: i64);
}

struct InsertionSortVector {
    list: Vec<i64>,
}

impl Sortable for InsertionSortVector {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }

    fn insert(&mut self, input: i64) {
        for count in 0..self.list.len() {
            if &input < &self.list[count] {
                self.list.insert(count, input);
                return;
            }
        }
        self.list.push(input);
    }
}

struct QuickSortVector {
    list: Vec<i64>,
}

impl Sortable for QuickSortVector {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }

    fn insert(&mut self, input: i64) {
        self.list.push(input);
    }
}

impl QuickSortVector {
    fn sort(self) -> Vec<i64> {
        let length = self.list.len()-1;
        quicksort(self.list, 0, length as i64)
    }
}

fn quicksort(arr: Vec<i64>, start: i64, end: i64) -> Vec<i64> {
    if start < end {
        /* pi is partitioning index, arr[pi] is now
           at right place */
        let (mut arr, pi) = partition(arr, start, end);

        arr = quicksort(arr, start, pi - 1);  // Before pi
        arr = quicksort(arr, pi + 1, end); // After pi
        arr
    } else {
        arr
    }
}

fn partition(mut arr: Vec<i64>, start: i64, end: i64) -> (Vec<i64>, i64) {
    let pivot: i64 = arr[end as usize];

    let mut i: i64 = start;

    for j in start..end {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }

    arr.swap(i as usize, end as usize);
    (arr, i)
}

struct RadixSortVector {
    list: Vec<i64>,
}

impl Sortable for RadixSortVector {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }

    fn insert(&mut self, input: i64) {
        self.list.push(input);
    }
}