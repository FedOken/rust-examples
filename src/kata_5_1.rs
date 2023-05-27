use std::collections::HashMap;

pub fn binarray(a: &[u8]) -> u32 {
    let mut count = 0;
    let mut max_length:u32 = 0;
    let mut subarray_sums: HashMap<i32, i32> = HashMap::new();

    subarray_sums.insert(0, -1);

    for (i, &val) in a.iter().enumerate() {
        count += if val == 0 { -1 } else { 1 };
        if subarray_sums.contains_key(&count) {
            let length = (i as i32) - subarray_sums[&count];
            max_length = max_length.max(length as u32);
        } else {
            subarray_sums.insert(count, i as i32);
        }
    }

    // println!("{}", max_length);


    max_length
}

#[cfg(test)]
mod tests {
    use super::binarray;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u8], expected: u32) {
        assert_eq!(binarray(a), expected, "{ERR_MSG} with a= {a:?}")
    }

    #[test]
    fn fixed_tests() {
        for (input, expected) in [
                                  (vec![0,1], 2),
                                  (vec![0], 0),
                                  (vec![1,1,0,1,1,0,1,1],4),
                                  (vec![0,1,1,0,1,1,1,0,0,0],10),
                                  (vec![0,0,1,1,1,0,0,0,0,0],6),
                                    ] {
            dotest(&input, expected);
        }
    }
}
// cargo test -- kata_5_1 --nocapture