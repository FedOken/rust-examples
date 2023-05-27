#### Commands
Run test: `cargo test -- sha1 --nocapture`

####  Tests
1. `test_sha1_with_valid_hashes` - сompares hashes from the implementation with actual valid hashes.
2. `test_comapre_sha1_realization_32_with_sha1_from_lib` - сomparison of hashing speeds for an array of strings of different lengths. The number of strings and length can be changed in the test.
Here is the result of the comparison:
    ```
    Rust existed lib execution time for 500 words with length 10000 symbols: 178 ms
    Realization execution time for 500 words with length 10000 symbols: 396 ms
    ```