use std::{cmp::{max, min}, fs};

fn count_safe_part_1(file: &str) {
    let rows: Vec<&str> = file.split("\n").collect();
    let mut safe_counter: u32 = 0;
    'outer: for row in rows {
        if !row.is_empty() {
            let chars = row.split(" ").collect::<Vec<_>>();
            let nums_parsed = chars
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut ascending: bool = false;
            for (i, num) in nums_parsed.iter().enumerate() {
                if i == 0 {
                    let next = nums_parsed[i + 1];
                    let dif = max(next, *num) - min(next, *num);
                    if next == *num || dif > 3{
                        continue 'outer;
                    }
                    ascending = next > *num;
                }
                if i != 0 && i != nums_parsed.len() - 1 {
                    let next = nums_parsed[i + 1];
                    let dif = max(next, *num) - min(next, *num);
                    if *num == next {
                        continue 'outer;
                    }
                    if ascending && next < *num {
                        continue 'outer;
                    }
                    if !ascending && next > *num {
                        continue 'outer;
                    }
                    if dif > 3 {
                        continue 'outer;
                    }
                }
                if i == nums_parsed.len() - 1 {
                    let prev = nums_parsed[i -1];
                    let dif = max(prev, *num) - min(prev, *num);
                    if dif > 3 {
                        continue 'outer;
                    }
                    if ascending && prev > *num {
                        continue 'outer;
                    }
                    if !ascending && prev < *num {
                        continue 'outer;
                    }
                    safe_counter += 1;
                    println!("Safe row: {row:?}");
                }
            }
        }
    }
    println!("safe counter: {safe_counter}")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("input.real")?;
    count_safe_part_1(&file);
    Ok(())
}
