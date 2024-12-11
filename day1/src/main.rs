use std::{error::Error, fs};

fn distances(file: &str) -> i64 {
    let parsed_file: Vec<_> = file.split("\n").collect();
    let mut left_vector: Vec<i64> = Vec::new();
    let mut right_vector: Vec<i64> = Vec::new();

    for row in parsed_file {
        if !row.is_empty() {
            let row_values: Vec<&str> = row.split("   ").collect();
            match row_values.as_slice() {
                [left, right] => {
                    left_vector.push(left.parse().unwrap());
                    right_vector.push(right.parse().unwrap())
                }
                _ => continue,
            }
        }
    }
    let mut distances: Vec<i64> = Vec::new();
    left_vector.sort();
    right_vector.sort();
    for i in 0..right_vector.len() {
        if left_vector[i] >= right_vector[i] {
            distances.push(left_vector[i] - right_vector[i])
        } else {
            distances.push(right_vector[i] - left_vector[i])
        }
    }
    println!("{dist}", dist = distances.iter().sum::<i64>());
    distances.iter().sum::<i64>()
}

fn similarity(file: &str) -> i64 {
    let parsed_file: Vec<_> = file.split("\n").collect();
    let mut left_vector: Vec<i64> = Vec::new();
    let mut right_vector: Vec<i64> = Vec::new();
    for row in parsed_file {
        if !row.is_empty() {
            let row_values: Vec<&str> = row.split("   ").collect();
            match row_values.as_slice() {
                [left, right] => {
                    left_vector.push(left.parse().unwrap());
                    right_vector.push(right.parse().unwrap())
                }
                _ => continue,
            }
        }
    }
    let mut similarity: Vec<i64> = Vec::new();
    // Now, need to check how many times each number in left vector is found in right vector
    for num in left_vector.iter() {
        let count = right_vector.iter().filter(|x| *x == num).count();
        let res = num * count as i64;
        similarity.push(res);
    }
    println!("{sim}", sim = similarity.iter().sum::<i64>());
    similarity.iter().sum::<i64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_string: String = fs::read_to_string("input.test")?;
    distances(&file_string);
    similarity(&file_string);
    Ok(())
}
