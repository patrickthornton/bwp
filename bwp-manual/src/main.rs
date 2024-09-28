use std::iter::zip;

fn main() {
    // let nums: Vec<String> = [
    //     "0000", "0111", "0222", "0333", "0444", "1012", "1123", "1234", "1340", "1401", "2024",
    //     "2130", "2241", "2302", "2413", "3031", "3142", "3203", "3314", "3420", "4043", "4104",
    //     "4210", "4321", "4432",
    // ]
    let nums: Vec<String> = [
        "0000", "0111", "0222", "0333", "1012", "1103", "1230", "1321", "2023", "2132", "2201",
        "2310", "3031", "3120", "3213", "3302",
    ]
    .iter()
    .map(|elt| elt.to_string())
    .collect();

    if check(nums) {
        println!("YEAH");
    } else {
        println!("wah");
    }
}

fn check(nums: Vec<String>) -> bool {
    let b_2 = nums.len();
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
                println!("{} has {} in common with {}", num, commons, other_num);
                return false;
            }
        }
    }

    true
}
