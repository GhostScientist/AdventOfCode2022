use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    
    let reader = BufReader::new(file);

    let mut sums = Vec::new();
    let mut current_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            let num = line.parse::<i32>().unwrap();
            current_sum += num;
        }        
    }

    sums.sort();

    let largest = sums[sums.len() - 1];
    let second_largest = sums[sums.len() - 2];
    let third_largest = sums[sums.len() - 3];

    println!("Largest: {}", largest);
    println!("Second largest: {}", second_largest);
    println!("Third largest: {}", third_largest);
    println!("Sum: {}", largest + second_largest + third_largest);
    
    
}