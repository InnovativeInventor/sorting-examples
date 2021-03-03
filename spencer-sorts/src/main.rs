use std::io::BufRead;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "spencer-sorts", about = "A potpourri of sorting algorithms")]
struct Opt {
    #[structopt(short, long)]
    insertion: bool,

    #[structopt(short, long)]
    quick: bool,

    #[structopt(short, long)]
    radix: bool,

    #[structopt(short, long)]
    bubble: bool,
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
    } else if opt.radix {
        let mut sorter = RadixSortVector::new(cap);

        for line in input.lock().lines(){
            let num: i64 = line.unwrap().parse().unwrap();
            sorter.insert(num);
        }

        for integer in sorter.sort() {
            println!("{}", integer);
        }
    } else if opt.bubble {
        let mut sorter = BubbleSortVector::new(cap);

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
        quick_sort(self.list, 0, length as i64)
    }
}

fn quick_sort(arr: Vec<i64>, start: i64, end: i64) -> Vec<i64> {
    if start < end {
        /* pi is partitioning index, arr[pi] is now
           at right place */
        let (mut arr, pi) = partition(arr, start, end);

        arr = quick_sort(arr, start, pi - 1);  // Before pi
        arr = quick_sort(arr, pi + 1, end); // After pi
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

impl RadixSortVector {
    fn sort(self) -> Vec<i64> {
        radix_sort(self.list)
    }
}

fn radix_sort(mut arr: Vec<i64>) -> Vec<i64>{
    let length: f32 = arr.len() as f32;
    let digits: f32 = length.log(10 as f32).round();

    for i in 0..digits as i32 {
        let mut count = vec![0; 10];
        let mut ans = vec![0; length as usize];

        for num in &arr {  // counts number of (i+1)th digit
            count[((num % 10_i64.pow((i + 1) as u32)) / (10_i64.pow((i) as u32) as i64)) as usize] += 1;
        }
        for j in 1..10 {  // accumulates indexes
            count[j] += count[j-1];
        }
        for j in 0..length as i32 {  // places arr elements into bins in ans
            let digit = arr[(length as i32 -j-1) as usize] % 10_i64.pow((i + 1) as u32) / (10_i64.pow((i) as u32) as i64);
            count[digit as usize] -= 1;
            ans[count[digit as usize]] = arr[(length as i32 -j-1) as usize];
        }
        for j in 0..length as i32 {  // puts ans numbers back into arr
            arr[j as usize] = ans[j as usize]
        }
    }
    arr
}

struct BubbleSortVector {
    list: Vec<i64>,
}

impl Sortable for BubbleSortVector {
    fn new(cap: usize) -> Self {
        Self {
            list: Vec::<i64>::with_capacity(cap),
        }
    }

    fn insert(&mut self, input: i64) {
        self.list.push(input);
    }
}

impl BubbleSortVector {
    fn sort(self) -> Vec<i64> {
        bubble_sort(self.list)
    }
}

fn bubble_sort(mut arr: Vec<i64>) -> Vec<i64> {
    let length = arr.len();
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..length-1 {
            if arr[i] > arr[i+1] {
                sorted = false;
                arr.swap(i, i+1)
            }
        }
    }
    arr
}