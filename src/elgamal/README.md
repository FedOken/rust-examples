#### Overview
I was not able to complete the requirements completely due to lack of time, but I did the bulk of it features:

1. It is quite a problem to create a prime number with a length of 4096 bits, I managed to do only on the length of 2048 bits.
2. `primitive root of modulo` - this is not good at all. If you do a brute force, it is very difficult to calculate them. To implement a more complex algorithm, unfortunately no time, because of this p range is very limited.
3. I made an algorithm of usual encryption, not purposeful, what I managed, I managed.

#### Commands
Run test: `cargo test -- elgamal --nocapture`

####  Test result example
```
hex_num: 6699

p: 27457
g: 21
Private key: 861
Public key: 6237

a: 1304
b: 20503
m: 6699

r: 17524
s: 22071

test elgamal::elgamal::tests::test_all ... ok
```