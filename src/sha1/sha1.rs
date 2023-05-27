pub struct Sha1Realization32 {
    buffer: Vec<u8>,
    hash: Vec<u8>,
}

impl Sha1Realization32 {
    pub fn new() -> Self {
        Sha1Realization32 { buffer: vec![], hash: vec![] }
    }

    pub fn update(&self, input: &str) -> Self {
        let buffer = input.bytes().collect();

        Sha1Realization32 { buffer, hash: vec![] }
    }

    pub fn hash(&self) -> Self {
        if self.buffer.len() == 0 {
            panic!("Emty buffer, call 'update' before hash")
        }

        let mut buffer: Vec<u8> = self.buffer.clone();
        let original_length = buffer.len() as u64 * 8;

        // Join bit '1' to the message
        buffer.push(0x80);

        // Join k '0' bits, where k is the smallest number ≥ 0 such that the length of the resulting message
        // (in bits) is equal modulo 512 to 448 (length mod 512 == 448)
        while (buffer.len() * 8) % 512 != 448 {
            buffer.push(0);
        }

        // Add the length of the original message (before preprocessing) as a whole 64-bit
        // Big-endian number, in bits.
        let length_bytes: [u8; 8] = (original_length as u64).to_be_bytes();
        buffer.extend_from_slice(&length_bytes);

        // Variable initialization:
        let mut h0: u32 = 0x67452301;
        let mut h1: u32 = 0xEFCDAB89;
        let mut h2: u32 = 0x98BADCFE;
        let mut h3: u32 = 0x10325476;
        let mut h4: u32 = 0xC3D2E1F0;

        // In the process, the message is broken down sequentially by 512 bits:
        for chunk in buffer.chunks(64) {
            let mut words: [u32; 80] = [0; 80];

            // Initializing a word array
            for (i, chunk_bytes) in chunk.chunks(4).enumerate() {
                let mut word = [0u8; 4];
                word.copy_from_slice(chunk_bytes);
                words[i] = u32::from_be_bytes(word);
            }

            // 16 32-bit words are augmented to 80 32-bit words:
            for i in 16..80 {
                let word = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
                words[i] = word.rotate_left(1);
            }

            // Initializing the hash values of this part:
            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;

            // Main loop:
            for i in 0..80 {
                let f;
                let k;

                if i < 20 {
                    f = (b & c) | (!b & d);
                    k = 0x5A827999;
                } else if i < 40 {
                    f = b ^ c ^ d;
                    k = 0x6ED9EBA1;
                } else if i < 60 {
                    f = (b & c) | (b & d) | (c & d);
                    k = 0x8F1BBCDC;
                } else {
                    f = b ^ c ^ d;
                    k = 0xCA62C1D6;
                }

                let temp = a.rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(words[i])
                    .wrapping_add(k);

                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            // We add the hash value of this part to the result:
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        // Total hash value(h0, h1, h2, h3, h4 must be converted to big-endian):
        let hash = [
            h0.to_be_bytes(),
            h1.to_be_bytes(),
            h2.to_be_bytes(),
            h3.to_be_bytes(),
            h4.to_be_bytes(),
        ]
        .concat();

        Sha1Realization32 { buffer, hash }
    }

    pub fn to_hex(&self) -> String {
        self.hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::Instant;
    use sha1::{Sha1, Digest};

    fn generate_random_string(length: usize) -> String {
        let mut rng = rand::thread_rng();
        let alphabet: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
        let random_string: String = (0..length)
            .map(|_| {
                let index = rng.gen_range(0..alphabet.len());
                alphabet[index] as char
            })
            .collect();
    
        random_string
    }

    #[test]
    fn test_sha1_with_valid_hashes() {
        // Used https://emn178.github.io/online-tools/sha1.html for result check

        let sha1_dl = Sha1Realization32::new();

        assert_eq!(sha1_dl.update("Hello").hash().to_hex(), "f7ff9e8b7bb2e09b70935a5d785e0cc5d9d0abf0");
        assert_eq!(sha1_dl.update("hello").hash().to_hex(), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
        assert_eq!(sha1_dl.update("world").hash().to_hex(), "7c211433f02071597741e6ff5a8ea34789abbf43");
        assert_eq!(sha1_dl.update("Hello world").hash().to_hex(), "7b502c3a1f48c8609ae212cdfb639dee39673f5e");
        assert_eq!(sha1_dl.update("Програмна реалізація алгоритму гешування").hash().to_hex(), "60849d3ae9b32743c3d06693d14a2b08f561b87b");
        assert_eq!(
            sha1_dl.update("1 Теоретичні відомості. Сучасні блочні алгоритми гешування є важливою складовою криптографічних систем і використовуються для створення геш-значень, які є незворотніми ідентифікаторами вхідних даних. Одним з ключових аспектів сучасних блочних алгоритмів гешування є порядок обробки вхідних даних.").hash().to_hex(),
            "efdec6ff592d2cd154dbea402e56e0aee273fada"
        );
    }

    #[test]
    fn test_comapre_sha1_realization_32_with_sha1_from_lib() {
        let strings_count = 500;
        let string_length = 10000;
        let mut strings: Vec<String> = [].to_vec();

        for _ in 0..strings_count {
            strings.push(generate_random_string(string_length));
        }

        // let start = Instant::now();
        for word in strings.iter() {
            let mut hasher = Sha1::new();
            hasher.update(word);
            hasher.finalize();
        }
        // let end = Instant::now();
        // let duration = end - start;

        // let duration_ms = duration.as_millis();
        // println!("Rust existed lib execution time for {} words with length {} symbols: {} ms", strings_count, string_length, duration_ms);

        let sha1_dl = Sha1Realization32::new();

        // let start = Instant::now();
        for word in strings.iter() {
            sha1_dl.update(word).hash();
        }
        // let end = Instant::now();
        // let duration = end - start;

        // let duration_ms = duration.as_millis();
        // println!("Realization execution time for {} words with length {} symbols: {} ms", strings_count, string_length, duration_ms);
    }
}