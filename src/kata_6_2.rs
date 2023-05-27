use std::collections::HashMap;

pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut counts: HashMap<u8, u8> = HashMap::new();

    for num in lst {
        let entry = counts.get(num).unwrap_or(&0);

        if *entry < n as u8 {
            counts.insert(*num, entry + 1);
            res.push(*num);
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
        // assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}

// cargo test -- kata_6_2 --nocapture