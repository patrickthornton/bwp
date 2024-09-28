use std::iter::zip;

use itertools::Itertools;

fn main() {
    const B: usize = 4;
    const B_2: usize = B * B;

    let mut fixed: Vec<String> = Vec::new();
    for j in 0..B {
        fixed.push(
            "0".to_string()
                + j.to_string().as_str()
                + j.to_string().as_str()
                + j.to_string().as_str(),
        );
    }

    let mut prefixes: Vec<String> = Vec::new();
    for i in 1..B {
        for j in 0..B {
            prefixes.push(i.to_string() + j.to_string().as_str());
        }
    }

    let mut suffixes: Vec<String> = Vec::new();
    for i in 0..B {
        for j in 0..B {
            if i != j {
                suffixes.push(i.to_string() + j.to_string().as_str());
            }
        }
    }

    let suffix_perms = suffixes.iter().permutations(suffixes.len());
    for suffix_perm in suffix_perms {
        let nums_ending: Vec<String> = zip(&prefixes, suffix_perm)
            .map(|(str1, str2)| String::with_capacity(4) + str1 + str2)
            .collect();
        let nums: Vec<String> = fixed.iter().chain(&nums_ending).cloned().collect();
        println!("testing {:?}", nums);
        if check(&nums, B_2) {
            println!("Worked!!! with {:?}", nums);
            return;
        }
    }
}

fn check(nums: &Vec<String>, b_2: usize) -> bool {
    for i in 0..b_2 {
        for j in i + 1..b_2 {
            let num = &nums[i];
            let other_num = &nums[j];

            let mut commons = 0;
            for (c1, c2) in zip(num.chars(), other_num.chars()) {
                if c1 == c2 {
                    commons += 1;
                }
            }

            if commons >= 2 {
                return false;
            }
        }
    }

    true
}
