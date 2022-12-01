use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {

    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut largest_calories = -1;

    let mut current_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current_sum > largest_calories {
                largest_calories = current_sum;
            }
            current_sum = 0;
        } else {
            let num = line.parse::<i32>().unwrap();
            current_sum += num;
        }        
    }

    println!("{}", largest_calories);
}