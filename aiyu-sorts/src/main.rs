//extern crate rand;

//use rand::Rng;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Result,Lines};
use std::path::{Path, PathBuf};


fn swap(arr: &mut[i32], a:usize, b:usize){
    let par = arr[a];
    arr[a] = arr[b];
    arr[b] = par;
}

// quicksort partition
fn partition(arr: &mut[i32],l:usize,h:usize) ->usize{
    //println!("{}",h);
    let pivot = arr[h];
    let mut i:isize = l as isize;
    for j in l..h{
        if (arr[j as usize] < pivot){
            swap(arr,i as usize,j as usize);
            i = i+1;
        }
    }
    swap(arr,i as usize,h);
    return i as usize;

}

// quicksort with random pivot
/**
fn random_partition(arr: &mut [i32],l:usize,h:usize) -> usize{
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(l..h);
    swap(arr,r as usize,h as usize);
    return partition(arr,l,h);
}
*/


fn quicksort(arr: &mut [i32],l:usize,h:usize){
    //println!("{}",h);
    if (l<h){
        let p = partition(arr, l,h);
        //println!("{}",p);
        if p > 0{
            quicksort(arr,l,p-1);
        }
        quicksort(arr,p+1,h);
    }
}

fn merge(arr: &mut[i32],l:usize,p:usize,h:usize){
    let n1 = p-l+1;
    let n2 = h-p;
    let mut L = vec![0; n1];
    let mut R = vec![0; n2];
    for i in 0..n1{
        L[i] = arr[l+i];
    }
    for j in 0..n2{
        R[j] = arr[p+j+1];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = l;
    while (i<n1 && j<n2){
        if (L[i] <= R[j]){
            arr[k] = L[i];
            i = i+1;
        }else{
            arr[k] = R[j];
            j = j+1;
        }
        k = k+1;
    }
    while (i<n1){
        arr[k] = L[i];
        i = i+1;
        k = k+1;
    }
    while (j<n2){
        arr[k] = R[j];
        j = j+1;
        k = k+1;
    }
}

fn mergesort(arr: &mut[i32], l:usize, h:usize){
    if (l < h){
        let p = (l+h)/2;
        mergesort(arr,l,p);
        mergesort(arr,p+1,h);
        merge(arr,l,p,h);
    }
}

fn max_heapify(arr: &mut[i32], i:usize,n:usize){
    let l = 2*i + 1;
    let r = 2 *i + 2;
    let mut largest = i;
    if l<n && arr[l] > arr[largest]{
        largest = l;
    }
    if r<n && arr[r] > arr[largest]{
        largest = r;
    }
    if (largest != i){
        swap(arr,i,largest);
        max_heapify(arr,largest,n);
    }
}

fn heapsort(arr: &mut[i32],n:usize){
    for i in (0..n/2).rev(){
        //println!("i is {}",i);
        max_heapify(arr,i,n);
    }
    for i in (1..n).rev(){
        swap(arr,0,i);
        max_heapify(arr,0,i);
    }
}

// read lines directly from txt file
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main(){
    let mut vec = Vec::new();
    let input = std::io::stdin();

    /*
    if let Ok(lines) = read_lines("benchmark-2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    */



    for line in input.lock().lines(){
        //println!("{:?}",line);
        let num: i32= line.unwrap().parse().unwrap();
        vec.push(num);
    }

    let k: usize = vec.len()-1;
    quicksort(vec.as_mut_slice(), 0, k);
    //mergesort(vec.as_mut_slice(),0,k);
    //heapsort(vec.as_mut_slice(),k+1);

    for things in vec{
        println!("{}",things);
    }


    //println!("{:?}",vec);


}