use std::collections::HashMap;

pub fn meanmedmod(vals : &Vec<i32>) -> Option<(f32, i32, i32)> {
    let vals_cnt = vals.len();

    if vals_cnt == 0 {
        return None;
    }

    let mut occ = HashMap::new();

    let sum : i32 = vals.iter().sum(); 
    let mean = sum as f32 / vals_cnt as f32;

    for x in vals.iter() {
        let curr_occ = occ.entry(x).or_insert(0);
        *curr_occ += 1;
    }

    let mut moda : i32 = vals[0];
    let mut greatest = 0;
    for (key, value) in occ {
        if value > greatest {
            greatest = value;
            moda = *key;
        }
    }

    let mut vals_sorted = vals.clone();
    vals_sorted.sort();
    let med = vals_sorted[vals_cnt / 2];

    Some((mean, med, moda))
}
