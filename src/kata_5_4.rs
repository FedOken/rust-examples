pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut res = 0;

    for i in 0..seq.len() {
        if seq[i] <= 0 {
            continue;
        }

        let mut res_temp = 0;
        for n in 0..=i {
            res_temp += seq[i - n];

            if res_temp > res {
                res = res_temp;
            }

            if res_temp <= 0 {
                break;
            }
        }

        res_temp = seq[i];
        for n in (i + 1)..seq.len() {
            if res_temp > res {
                res = res_temp;
            }

            res_temp += seq[n];

            if res_temp <= 0 {
                break;
            }
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::max_sequence;
    
    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[10, 20, 30, 40]), 100);
        assert_eq!(max_sequence(&[1, 0, 0, 0]), 1);
        assert_eq!(max_sequence(&[0, 0, 0, 1]), 1);

        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    } 
}

// https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust
// cargo test -- kata_5_4 --nocapture