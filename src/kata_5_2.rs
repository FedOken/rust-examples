use std::collections::HashMap;

pub fn decomp(n: i32) -> String {
    let mut prime_all = vec![2, 3, 5, 7];
    for i in (11..=n/2).step_by(2) {
        if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            continue;
        };

        prime_all.push(i);
    }
    let prime_last = prime_all[prime_all.len() - 1];

    let mut primes_count: HashMap<i32, i32> = HashMap::new();
    for num in 2..=n {
        let mut k = num;
        while k != 0 {

            if k == 1 { break }

            for prime in prime_all.iter() {
                let modulo = k % prime;
                if modulo == 0 {
                    increment_hashmap(&mut primes_count, *prime);
                    k = k / prime;
                    break
                }

                if *prime == prime_last {
                    increment_hashmap(&mut primes_count, k);
                    k = 1;
                }
            }
        }
    }

    let mut primes_count_sorted: Vec<_> = primes_count.iter().collect();
    primes_count_sorted.sort_by_key(|a| a.0);

    let mut result = String::new();
    for (num, count) in primes_count_sorted {
        if *count == 1 {
            result = format!("{} * {}", result, num);
        } else {
            result = format!("{} * {}^{}", result, num, count);
        }
    }

    result = format!("{}", &result.trim()[2..]);
 
    result
}

pub fn increment_hashmap(map: &mut HashMap<i32, i32>, v: i32) {
    match map.get(&v) {
        Some(count) => {
            map.insert(v, count + 1);
        }
        None => {
            map.insert(v, 1);
        }
    }
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");

    }    
}

// Source: https://www.codewars.com/kata/5a045fee46d843effa000070
// cargo test -- kata_5_2 --nocapture