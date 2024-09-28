use std::iter::zip;

fn main() {
    // pattern from kyra mo: 0000, 0111, 0222...
    // 1012, 1123, 1234, ...
    // 2024, 2130, 2241, ...
    // for instance in base 5
    const NUM_GROUPINGS: usize = 100;

    for base in (3..=303).step_by(2) {
        let groupings = get_groupings(base, NUM_GROUPINGS);

        if check(groupings) {
            println!("woo!!!! for base {}", base);
        } else {
            println!("boo.... for base {}", base);
        }
    }
}

fn get_groupings(base: usize, grouping_size: usize) -> Vec<Vec<usize>> {
    let mut groupings: Vec<Vec<usize>> = Vec::with_capacity(base * base);

    for i in 0..base {
        for j in 0..base {
            let mut grouping: Vec<usize> = Vec::with_capacity(grouping_size);
            grouping.push(i);

            let mut offset = j;
            for _ in 0..grouping_size - 1 {
                grouping.push(offset % base);
                offset += i;
            }

            groupings.push(grouping);
        }
    }

    groupings
}

fn check(groupings: Vec<Vec<usize>>) -> bool {
    let len = groupings.len();
    for i in 0..len {
        for j in i + 1..len {
            if zip(&groupings[i], &groupings[j])
                .filter(|(c1, c2)| c1 == c2)
                .count()
                >= 2
            {
                return false;
            }
        }
    }

    true
}
