pub fn perimeter(n: u64) -> u64 {
    let mut prev = 1;
    let mut cur = 0;
    let mut res = 0;

    for _ in 0..=n {
        cur += prev;
        prev = cur - prev;

        res += cur * 4;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }

    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }  
}

// https://www.codewars.com/kata/559a28007caad2ac4e000083/train/rust
// cargo test -- kata_5_3 --nocapture