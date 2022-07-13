```
cargo test
   Compiling repro v0.1.0 (/home/matthew/git/repro)
    Finished test [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests (target/debug/deps/repro-b535a5ace3a0c059)

running 2 tests
test tests::validate_u8_deserializes ... ok
test tests::validate_mixed_deserializes ... FAILED

failures:

---- tests::validate_mixed_deserializes stdout ----
Result [1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]
thread 'tests::validate_mixed_deserializes' panicked at 'assertion failed: `(left == right)`
  left: `TestMixed { a: 1, b: 1, c: 1, d: 1, e: 1, f: 1, g: 1, h: 1 }`,
 right: `TestMixed { a: 1, b: 1, c: -1, d: 0, e: -1, f: 0, g: -1, h: 0 }`', src/lib.rs:42:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::validate_mixed_deserializes

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```
