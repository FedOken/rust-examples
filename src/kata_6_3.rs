pub fn solution(num: i32) -> i32 {
    let mut sum = 0;

    let mut num_3 = 0;
    let mut num_5 = 0;

    while num_3 < num || num_5 < num {
        num_3 += 3;
        num_5 += 5;

        sum += if num_3 < num {num_3} else {0};
        sum += if num_5 < num && num_5 % 3 != 0 {num_5} else {0};
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::solution;
    
    #[test]
    fn sample_tests() {
    //   assertion(expected, input);
      assertion(  23,   10);
      assertion(  33,   11);
      assertion( 225,   33);
      assertion(   8,    6);
      assertion(3420,  123);
      assertion( 543,   50);
      assertion(   0,    0);
      assertion(   0, -203);
      assertion(25719750, 10500);
    }
    
    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}

// cargo test -- kata_6_3 --nocapture