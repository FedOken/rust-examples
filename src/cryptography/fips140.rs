use std::collections::HashMap;
use num_traits::pow;

pub fn fips140(
    bits: &Vec<u8>,
    max_monobits: [u16; 2],
    max_series_count: HashMap<u16, [u16; 2]>,
    max_serries_length: u16,
    max_pocker_coef_range: [f32; 2]
) -> bool {
    let zero_monobits = calc_zero_monobit(&bits);
    // Check monobits
    if !(max_monobits[0] <= zero_monobits && zero_monobits <= max_monobits[1]) {
        panic!("Too many monobits: {}. Available from {} to {} monobits", zero_monobits, max_monobits[0], max_monobits[1])
    }
    // END

    // START check max series length
    let (series_count, biggest_serries_length) = calc_series_count(&bits);

    if biggest_serries_length > max_serries_length {
        panic!("Too big series: {}. Max available: {}", biggest_serries_length, max_serries_length);
    }
    // END

    // START check series count, without last
    let mut keys = max_series_count.keys().collect::<Vec<_>>();
    keys.sort();
    let repetition_count_to_sum_from = *keys.last().unwrap();

    let mut other_repetition_count = 0;
    for (repetition_count, entrances_count) in &series_count {
        if repetition_count >= repetition_count_to_sum_from {
            other_repetition_count += entrances_count;
            continue;
        }

        let valid_entrance_range = max_series_count.get(repetition_count).unwrap_or(&[0,0]);

        if !(valid_entrance_range[0] <= *entrances_count && *entrances_count <= valid_entrance_range[1]) {
            panic!(
                "Series #{} isn't in range [{},{}], entry count: {}",
                repetition_count, valid_entrance_range[0], valid_entrance_range[1], entrances_count
            )
        }
    }
    // END

    // START check last series
    let valid_entrance_range = max_series_count.get(&repetition_count_to_sum_from).unwrap_or(&[0,0]);
    if !(valid_entrance_range[0] <= other_repetition_count && other_repetition_count <= valid_entrance_range[1]) {
        panic!("Last series isn't in range [{},{}], entry count: {}", valid_entrance_range[0], valid_entrance_range[1], other_repetition_count)
    }
    // END

    // START check Pocker coefficient
    let pocker_coef = calc_pocker_coeff(&bits, 4);
    if !(max_pocker_coef_range[0] <= pocker_coef && pocker_coef <= max_pocker_coef_range[1]) {
        panic!("Pocker coefficient isn't in range: [{},{}]. Actual value: {}", max_pocker_coef_range[0], max_pocker_coef_range[1], pocker_coef);
    }
    // END

    return true;
}

fn calc_zero_monobit(bits: &Vec<u8>) -> u16 {
    let mut zero_bits_count: u16 = 0;

    for bit in bits.iter() {
        if *bit == 0 {
            zero_bits_count += 1;
        }
    }

    zero_bits_count
}

fn calc_series_count(bits: &Vec<u8>) -> (HashMap<u16, u16>, u16) {
    let mut length: u16 = 0;
    let mut counts: HashMap<u16, u16> = HashMap::new();

    let mut start_bit: u8 = 2;
    let mut length_current: u16 = 0;

    for (i, _) in bits.iter().enumerate().skip(1) {
        if bits[i - 1] != bits[i] {
            if start_bit != bits[i] {   // Start series calculation 10..
                start_bit = bits[i - 1];
                length_current += 1;
            } else {                    // Finish series calculation 10..01, and start new
                let series_count_current = counts.get(&length_current).unwrap_or(&0);
                counts.insert(length_current, series_count_current + 1);

                if length_current > length {
                length = length_current;
                }
        
                start_bit = bits[i - 1];
                length_current = 1;
            }
        } else {
            length_current += 1;         // Continue series calculation
        }
    }

    (counts, length)
}

fn calc_pocker_coeff(bits: &Vec<u8>, m: usize) -> f32 {
    let mut matches: HashMap<Vec<u8>, u32> = HashMap::new();

    for chunk in bits.chunks(m) {
        let partition = chunk.to_vec();
        *matches.entry(partition).or_insert(0) += 1;
    }

    let mut matches_exp_sum: f32 = 0.0;
    for value in matches.values_mut() {
        *value *= *value;

        matches_exp_sum += *value as f32;
    }

    let k: f32 = (bits.len() / m) as f32;
    let x3: f32 = pow(2, m) as f32 / k * matches_exp_sum - k;

    x3
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rand::Rng;
    use num_bigint::BigUint;
    use num_traits::Zero;

    // fn generate_big_value() -> BigUint {
    //     let mut rng = rand::thread_rng();

    //     let mut value: BigUint = BigUint::zero();
    //     let mut previous_bits: u8 = 0;
    //     let mut consecutive_count: u8 = 0;

    //     for _ in 0..2500 {
    //         let random_byte: u8 = rng.gen_range(0..=255);
    //         value <<= 8;
            
    //         if random_byte == previous_bits {
    //             consecutive_count += 1;
    //             if consecutive_count > 3 {
    //                 // Reset the consecutive count by generating a new random byte
    //                 let new_random_byte: u8 = rng.gen_range(0..=255);
    //                 value += BigUint::from(new_random_byte);
    //                 consecutive_count = 0;
    //                 previous_bits = new_random_byte;
    //             } else {
    //                 value += BigUint::from(random_byte);
    //                 previous_bits = random_byte;
    //             }
    //         } else {
    //             value += BigUint::from(random_byte);
    //             consecutive_count = 0;
    //             previous_bits = random_byte;
    //         }
    //     }

    //     value
    // }

    fn biguint_to_bits(number: &BigUint) -> Vec<u8> {
        let mut bits = Vec::new();
        let zero = BigUint::zero();
    
        if number == &zero {
            bits.push(0);
            return bits;
        }
    
        let bytes = number.to_bytes_be();
        for byte in bytes {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1u8;
                bits.push(bit);
            }
        }

        bits
    }

    #[test]
    fn test_fip140() {
        let mut max_series_count: HashMap<u16, [u16; 2]> = HashMap::new();
        max_series_count.insert(1, [2267,5000]);
        max_series_count.insert(2, [1079,2500]);
        max_series_count.insert(3, [502,1300]);
        max_series_count.insert(4, [223,600]);
        max_series_count.insert(5, [90,350]);
        max_series_count.insert(6, [90,350]);

        // Use this code to generate valid number
        // for _ in 0..1000 {
        //     let big_number = generate_big_value();
        //     let bits = biguint_to_bits(&big_number);
        //     let hex = big_number.to_str_radix(16);

        //     let max_series_count_clone = max_series_count.clone();

        //     let result = std::panic::catch_unwind(|| {
        //         fips140(&bits, [9654,10346], max_series_count_clone, 13, [1.03,57.4]);
        //     });
        
        //     match result {
        //         Ok(_) => {
        //             println!("Function call completed successfully.");
        //             println!("{}", hex);
        //             break;
        //         }
        //         Err(e) => {}
        //     }
        // }

        // Hex generated with code above
        let hex_string = "b5f1b183cceb66b0c22d1fce1436030cca17235841a568cd325a915d347619da1447c1dc7b1694cb7387129f59c24e6ff2e9a3f1d754cd1190f6c2230d15bc2b2c701646cd6f0928d38ca6eff5e71d74b72483ad6138c812099667d5448089fc7a7d077100fe67d26a3e98c72670dc01ae1a5381f6beb01f2bfc291b208140e2cc52595f1e4843c1543b8796eda84226ff39196bdd8ceb88df2f1172131625cf6e16cb1d274a3a529c63ba35b40a6a9462ed6c6bdbb6891bd039891daa9bf2d2eb6964b124b6d9022c7857f10afc0ec06a9ab90847917f8ee668182ffb352cede0b716f9152ae3904a88d0be33087a2adc8b64d5e74187f9209735808226df9c6e175fb99785287d2363c75c642e326d5198aeb522524f672b9b2afa3a666d16255ae04211f1e1fbdf67b8bd21327fa0ec7d515a0a381c99ffe82a43b80b00761c1bf29191cb6554024351bc88682422974c57d04477f6ca45c1121d39ac81a52c2b9e908e3c08d11643f66543a0bda0af490ff3c658ca2e5caff278f315bf8a45d44156b41c61ff3b941804b1a50406dc4ad4b6505c1c08429ce83ccebe9a2fb9ccf2f4f8831016196977ee43beeb90c125583530f322f9abb2cbc9554a4fcf3538f9c0639ee2595abdc7470ae2facba5662a4abf850e81a66197f89afd7393b76b4f01750c5c79c4f92d9e9acfc913ca6d1347b46efbb024c5d544105e1ff9ce174806dbdd957484ca9de1f618e46584a2cf8fd2532ddb4ff5a7facf74b055553402d46bdeb9b569eee423fa2b2fdf001f68d17d3ea42658a02cdeff53512886762ba7480e891efb086f59a44aa951130515aadf098131e7a9a75733b15c3f2da909cef063edc812ea81e2e5e0126c4b21b2bbbdb777108b0bb3bcddf78718202c40fafcb7e10afbd9a36e54db2d4e3db8224f80fb524cdf1f13cc94e5bdb6b6ef809f893e117272120945549a2382831023e4282a3eea2ff6042cef06f4cba9aca2d2b411ea4d42f4a8a95a1cb12c1007c0ed0296a4ce38f9a09720852d8756403fbaf5ff21774803f2fe3e4c0fb627f6df792d1d1a7f04874c340fb051f616afed2c291e65039f6ff83327d899c7b1f323eb4a8770ea00e03f2e9566b9e795fab4164f4f82415203206d1d79a7cd69409611495782de962169107b0a54f08c85d28c177f483fdc2b5266955b219a08143119167233d953e95aeb851b76a38cedb03cc74739dc25e57adde073898ac5fbbd56e6b970da763207ad7bb98beccba8327faf5a29d0cbc8a82728a49dfb558d884e7b84d3163b1b65b7c7cee7ed219eea1fd000b5b6e6e4bf1dc6e1c5207c50203d1bb129fd7062201b8018d58427fd5a1cdc70d9776ad77a2df4493a18b45b6f43bba363bdc43be926b7a63946dd5e627c9fefacf712f00bf2f0ac3b891e503d5996440acb530cefca2143d392567040effd0974a37e9b2b5fdc9c0aa79993744031e8edc658c7f0383b3f63dff6e95181b7a926f50eba55ad0a56b4e54efb656e915643554020cb949a69e591c5eda4176fa925836c03fa237fd9fba9c96331442a755ff1b4f5806b0bc8811fd53bef49eb05cd9a47693402d1d31b396105a0e9aa2ddb30646bb1e949dcddd07d900d3a292bda702cfe514a1ba00b193496495be0a0ca113afec0dbb5859731af062210ccac9c7fb067639e62f1bae53ca4518927d6e00153ee98c4c7f6a884167fb9ddc099e41f11dd07281dfad1b9a2488c4bac58cb95bc106330b8051cfdaae478e6b7ebcf518401513e74400f6789c93ef1565cd1a8faa056f736c35379284ad8e80c07d25018f4652174cc88ac3448d556be75863a4145ad269b517d6679f2f4b8c9743276d9e7dfc31f23cf1830f01df1a4a31fbf02cfc9565983415e2120b9082cea3d18198ed2f15f6b9c5ce292ecd1add4e08ecc177ec019713f374a659980ea7f9dbb14e7b22fe6b907ac3d117d86b331bbc356817ec869a2e788d3b282f7499bfcf42c0fd3242c587c524cf1b17ecd718190b9efbd75355eca3d060fc3e55f23b318417512d6ae959c88e0326da49c6349b0500a45ffb8b40ea90becc2c93922b54d3521f6365f56f9fa9e9455f8b3a6bd0ca81e5ee641b331cffed6ca90d5642ee84fde83f40503c4a6b7a740be2305d4e325577e5923837c8e609e3a229457db60c3c834b1437d9e9a9a723f0757438e4d60218bb3f4303019acda6ff81dda6e21705f54c2de5db7830fe220c483bb784044626af7794e60cf578afe2c4053b3d73bff7ec17f5aa19703499193f3936ea6c2a8669d2a78e1adc2ea08e8bfa9db70dad110ef5573c52b7904f6a6f07a5d227464b54914942ee8088c0566838497dba75034e67369b3f5d8e53f58532c82fd51487480579bb249d6f40995c69b92e1476c8a57eddd3937fa9950942a309bba17f88baeed0d5f274f91e87443306a0bc41bb78a5e504ff7d6be4cba93b623ba0f1941a8788df0f6653d587ebbdf539e4eb2cc2c43fe22b0cef92efc9f43cc44b8cec27141003e8dbd4c99d94d3b9d46e487764859df39899d5e9a9a349dec2a846982782cf926d19ff3d517e809e243073c3f2791297c116ecacd04db5b3611da3fb1200d6884db73419972e5c067d0208367b02bcfad34aca6bfe1833de89be5492142ea3b2b5c001d6b7647fa3da48789a3a03890de990fb44eaa38c9d8e25b6fa9568a544da31df852df9e91cf6c9e86a02364e4886b79bbff812c735b35a801acc3c772d1ee0537255e7ce4f3667b2e707008ef6507b9b8fd86f11d47cc03f44835d374e7c3ea8ce293e1c3fce55f6f0448f82fb1bc6ff96479cf91e0c3dbdebb6f137499a060ee526d84774d372bc546cff6935997f70235458f1f8cd049abf404e30180c9485362bde7b2fff6ede9d56585a13f4c34d5d3261accb0562140e175cfdce01e2341605cffdeba6190e87275c1cabcb8d87505274d2f0cb12fe2e63d27c24bc1fc157ee27cb29bc957adb7c93361f875ce665b9b95037d68635346a63a37f361a6e18f5ed666c85f0375147b06d5cf95092efdc647771360509b1ab3cd558f59ce601472846ecc296e5111972ee63bc41667ac2d9b70b79fd7794feca11da0fdb048eebeef0dfa438a3a66f0ce725d1ced5f01dc9e0d6f345449885674f6c2b1aadec2ad47f19f5792d87cdb96f98addb28ba40e3c6d7b3be2e15e014fa8878a8389ca9f0f1e498178063df0dbc7620cf4c7843d528913bb25a48531414ee329a0bc86326a0014e504b40c59542301a48a8fb73a0948e44fd83dbfc7ddbe11bd58a807674d6f1f85d54eaa0dd9dd8fed5f5f5d0d23c7a462ad565d9f1fae886d3ef9a4ab4c19a82a67da758e8eb6148ed5fcdc845ec90fc3ac9f9fa4184cbd2f47391cd52ff1050a21dba2bee13e0e532e99c5c5395904c5411d7b86bc9ccfc82e5ce721b63619ac76abb385cc010f568bbb393e4f2995a3feb4c351cd17c1e56f6e9d374dd8669a3cdbe32dbcc3f58b1fb357d56e99ea2e2ffe0270115bbb44fe194b01";
        let big_number = BigUint::parse_bytes(hex_string.as_bytes(), 16).unwrap();
        let bits = biguint_to_bits(&big_number);

        let result = fips140(&bits, [9654,10346], max_series_count, 13, [1.03,57.4]);
        assert_eq!(bits.len(), 20000);
        assert_eq!(result, true);

    }

    #[test]
    fn test_calc_zero_monobit() {
        assert_eq!(calc_zero_monobit(&vec![0,0,0,0,0]), 5);
        assert_eq!(calc_zero_monobit(&vec![1,1,0,0,1]), 2);
        assert_eq!(calc_zero_monobit(&vec![0,0,0,0,1]), 4);
    }

    #[test]
    fn test_calc_series_count() {
        let (series_count, biggest_serries_length) = calc_series_count(&vec![1,0,1,1,0,0,0,1]);
        let series_1_bits = series_count.get(&1).unwrap_or(&0);
        let series_2_bits = series_count.get(&2).unwrap_or(&0);
        let series_3_bits = series_count.get(&3).unwrap_or(&0);
        let series_4_bits = series_count.get(&4).unwrap_or(&0);
        let series_5_bits = series_count.get(&5).unwrap_or(&0);
        let series_6_bits = series_count.get(&6).unwrap_or(&0);

        assert_eq!(*series_1_bits, 1);
        assert_eq!(*series_2_bits, 1);
        assert_eq!(*series_3_bits, 1);
        assert_eq!(*series_4_bits, 0);
        assert_eq!(*series_5_bits, 0);
        assert_eq!(*series_6_bits, 0);
        assert_eq!(biggest_serries_length, 3);

        let (series_count, biggest_serries_length) = calc_series_count(&vec![1,0,0,1,0,1,0,1,0,0,0,0,0,1,0,1]);
        let series_1_bits = series_count.get(&1).unwrap_or(&0);
        let series_2_bits = series_count.get(&2).unwrap_or(&0);
        let series_3_bits = series_count.get(&3).unwrap_or(&0);
        let series_4_bits = series_count.get(&4).unwrap_or(&0);
        let series_5_bits = series_count.get(&5).unwrap_or(&0);
        let series_6_bits = series_count.get(&6).unwrap_or(&0);

        assert_eq!(*series_1_bits, 7);
        assert_eq!(*series_2_bits, 1);
        assert_eq!(*series_3_bits, 0);
        assert_eq!(*series_4_bits, 0);
        assert_eq!(*series_5_bits, 1);
        assert_eq!(*series_6_bits, 0);
        assert_eq!(biggest_serries_length, 5);

        let (_, biggest_serries_length) = calc_series_count(&vec![1,0,0,0,0,0,1]);
        assert_eq!(biggest_serries_length, 5);
        let (_, biggest_serries_length) = calc_series_count(&vec![1,0,0,1,0,0,1]);
        assert_eq!(biggest_serries_length, 2);
        let (_, biggest_serries_length) = calc_series_count(&vec![1,0,0,0,0,1,1]);
        assert_eq!(biggest_serries_length, 4);
        let (_, biggest_serries_length) = calc_series_count(&vec![0,0,0,0,0,1,1]);
        assert_eq!(biggest_serries_length, 0);
        let (_, biggest_serries_length) = calc_series_count(&vec![1,0,0,1,1,1,1,0]);
        assert_eq!(biggest_serries_length, 4);
    }

    #[test]
    fn test_calc_pocker_coef() {
        // Test from example https://core.ac.uk/download/pdf/48404916.pdf
        let coef = calc_pocker_coeff(&vec![0,1,0,0,0,0,1,0,1,0,0,1,0,1,1,1,1,0,0,0,0,0,1,0,0,1,1,1,0,1,0,0,0,0,0,0,1,0,1,0,0,0,0,0,1,0,0,0,1,0], 2);
        assert_eq!(coef, 5.879999);
    }

    #[test]
    fn test_fip140_small_vector() {
        let mut max_series_count: HashMap<u16, [u16; 2]> = HashMap::new();
        max_series_count.insert(1, [0,100]);
        max_series_count.insert(2, [0,100]);
        max_series_count.insert(3, [1,3]);

        let res: bool = fips140(&vec![1,0,1,1,0,1,0,0,0,0,1,0,0,0,1,1,1,1,1,1,1,0], [1,100], max_series_count, 120, [1.0,40.0]);
        assert_eq!(res, true);
    }
}