// Given a list of integers, use a vector and return:
// - mean (the average value), 
// - median (when sorted, the value in the middle position), 
// - mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    // Alternative syntax:     
    //   let v: Vec<i32> = Vec::new();
    let mut vector_of_integers = vec![1, 1, 2, 3];

    // O(n log(n)) worst case performance, based on timsort.
    vector_of_integers.sort();

    println!("The mean is: {}", calculate_mean(&vector_of_integers));
    println!("The median is: {}", calculate_median(&vector_of_integers));
    println!("The mode is: {}", calculate_mode(&vector_of_integers));
}

// O(n) time complexity, can be combined with calculate_mode for 
// further optimisation. Since we can calculate both in a single loop.
fn calculate_mean(vector: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;

    for number in vector {
        sum += number;
    }

    sum as f32 / vector.len() as f32
}

fn calculate_median(vector: &Vec<i32>) -> f32 {
    if vector.len() % 2 == 0 {
        // Add two middle values together and return the average
        return (vector[vector.len() / 2] as f32 + vector[vector.len() / 2 - 1] as f32) / 2 as f32;
    }
    vector[vector.len() / 2] as f32
}

// O(n) time complexity, since we have to iterate over the vector...
fn calculate_mode(vector: &Vec<i32> ) -> i32 {
    let mut frequency_counter = HashMap::new();

    // A named struct would be nicer here
    let mut most_common_int: (i32, u32) = (0, 0);

    for number in vector {
        let count = frequency_counter.entry(number).or_insert(0);
        *count += 1;
        if *count > most_common_int.1 {
            most_common_int = (*number, *count);
        }
    }

    return most_common_int.0;
}

