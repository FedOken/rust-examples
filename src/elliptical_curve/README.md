#### Overview
Tasks other than serialization, deserialization, and displaying points on a curve have been done. There is an example of operations on points in the tests. Output to the screen can be done by trivial `println!`. I couldn't find good and simple solutions for working with points on a curve in existing libraries, so I wrote my own.

ECDH protocol testing implemented.

ECDSA sign and verification implemented.

#### Commands
Run test: `cargo test -- ec_wrapper --nocapture`

####  Test result example
```
running 3 tests
test elliptical_curve::ec_wrapper::tests::test_points ... ok
test elliptical_curve::ec_wrapper::tests::test_diffie_hellman ... ok
test elliptical_curve::ec_wrapper::tests::test_ecdsa_sign ... ok
```