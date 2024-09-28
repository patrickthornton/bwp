use std::{collections::HashMap, iter::zip, ops::Add};

use anyhow::Result;
use csv::Writer;

#[derive(Debug)]
struct Num {
    orig: String,
    edit: String,
}

fn main() -> Result<()> {
    const N: usize = 4;

    // row by column; i.e. mat[2] is the third row
    let mut mat: Vec<Vec<String>> = Vec::new();

    for i in 0..N {
        mat.push(add_row_with_prefix(i, N))
    }
    for i in 0..(N - 1) {
        mat.push(add_row_with_prefix(i, N))
    }

    let mut all_nums: Vec<Num> = Vec::new();
    for i in 0..N {
        for j in 0..N {
            let num_str = i.to_string() + j.to_string().as_str();
            let num = Num {
                orig: num_str.clone(),
                edit: num_str,
            };
            all_nums.push(num);
        }
    }

    // traverse
    // for each partition of nums that share no digits
    for i in 0..(N - 1) {
        // for each subset in the partition
        let mut suffixes: HashMap<String, usize> = HashMap::new();
        for j in 0..N {
            suffixes.insert(mat[j][0].to_owned(), j);
            // for each element in the subset
            for k in 1..N {
                suffixes.insert(mat[j + k][i + k].to_owned(), j);
            }
        }

        for num in &mut all_nums {
            num.edit = num.edit.clone()
                + suffixes
                    .get_mut(&num.orig)
                    .expect("should be there")
                    .to_string()
                    .as_str();
        }
    }

    let nums: Vec<String> = all_nums.iter().map(|elt| elt.edit.clone()).collect();

    for num in &nums {
        println!("{}", num);
    }

    check(nums)?;

    Ok(())
}

fn add_row_with_prefix(prefix: usize, n: usize) -> Vec<String> {
    let mut row: Vec<String> = Vec::new();
    for i in 0..n {
        row.push(i.to_string())
    }
    for i in 1..n {
        row.push(i.to_string())
    }
    row.iter().map(|elt| prefix.to_string() + elt).collect()
}

fn check(nums: Vec<String>) -> Result<()> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let num = &nums[i];
            let other_num = &nums[j];

            let mut commons = 0;
            for (c1, c2) in zip(num.chars(), other_num.chars()) {
                if c1 == c2 {
                    commons += 1;
                }
            }

            if commons >= 2 {
                println!("{} has {} in common with {}", num, commons, other_num);
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn save_to_csv(nums: Vec<String>) -> Result<()> {
    // save to csv
    let mut wtr = Writer::from_path("cards.csv")?;

    wtr.write_record(&["Guest", "Group 1", "Group 2", "Group 3", "Group 4"])?;
    let mut guest_id = 1;
    for num in nums {
        let mut record: Vec<String> = Vec::new();
        record.push(guest_id.to_string());
        for i in 0..4 {
            record.push(
                num.chars()
                    .nth(i)
                    .expect("should be there")
                    .to_digit(10)
                    .expect("should be a digit")
                    .add(1)
                    .add(10 * (i as u32))
                    .to_string(),
            );
        }
        wtr.write_record(record)?;
        guest_id += 1;
    }

    Ok(())
}
