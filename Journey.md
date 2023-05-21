# Writing the morris code program with Rust

## First steps

I know I need a "table" that translates ASCII to Morris Code, so I went with a HashMap, where the key is the ASCII character and the value is the dashes and dots. 

Before branching out into the world of crates and separate files I put everything in one file to start with. The first iteration just loads the map and prints out two hard coded values.

The `morris_code_table` is a Vec of String types. Why? Because neither the value nor the keys will need to be changed. 

Added four tests, because that exactly what should be done.

## Step Two – Splitting things up

That all works. Now I want to split the `morris_code_table` to it’s own crate. My thinking is to create a file called models.rs to house “modeled” data, like the Morris Code Table. In order to do that I create a lib.rs and add `pub mod models;`. Next I move the HashMap table to models.rs making sure I give it the proper visibility. Then I add the following to main.rs.

```rust
pub mod models;

use crate::models::morris_code::morris_code_table;
```

Running `cargo test` reveals good things!

```bash
❯ cargo run
   Compiling morris v0.1.0 (/home/rgeorgia/workspace/rust_projects/morris_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/morris`
Quote is .-..-.
Capitable A is .-

morris_code on main [!?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo test
   Compiling morris v0.1.0 (/home/rgeorgia/workspace/rust_projects/morris_code)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/main.rs (target/debug/deps/morris-b7735657e06f6f52)

running 4 tests
test tests::test_morris_code_not_empty ... ok
test tests::test_letter_a ... ok
test tests::test_morris_code_table_length_53 ... ok
test tests::test_number_42 ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```