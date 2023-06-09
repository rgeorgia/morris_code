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

## Step Three – get input and build a result

I guess I’ll go with [clap](https://docs.rs/clap/4.3.0/clap/). Why? The project still seems to be active and all the cool kids use clap.
The plan is to enter a letter, word or sentence as a command-line argument. When you press `enter` morris_code will display the code for what you typed in. Example:

```bash
$ morris_code The answer is 42
- .... . / .- -. ... .-- . .-. / .. ... / ....- ..---

```

I get the input from the command-line, which is the code field from the Args struct. This returns and Option. I could try to unwrap the `args.code` without an input value, the “app” panics. I don’t want that. I want to “catch” the error and exit gracefully. So I do a `let input = match` to unwrap `args.code`. If there’s no value, then I print an error message an exit. I guess there maybe better ways to handle this, like

```rust
use clap::{command, Arg, ArgAction};
```

The build_code function takes the input string, iterates over each character building a string of corresponding dots and dashes.

## Summary for the first three steps

Get user input. The input is of String type, even if it’s a single character. Pass a reference of that String to the build_code function. The build_code function takes a string slice and returns a String. We create a new HashMap from the `models::morris_code::morris_code_table` function in models.rs. We also create a mutable String reference called result. It’s a mutable reference because we will be changing it’s value by pushing Strings onto it. Now we iterate over the characters in the `input_str` string slice. The item serves as a key to get the value from our HashMap. We push the value onto our result string then return the result.

## All test pass, but it's time to write more tests

```bash
❯ cargo test
   Compiling morris v0.1.0 (/home/rgeorgia/workspace/rust_projects/morris_code)
    Finished test [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests src/main.rs (target/debug/deps/morris-fcb5527285925d70)

running 7 tests
test tests::test_build_code_letter ... ok
test tests::test_build_code_word ... ok
test tests::test_build_code_sentance ... ok
test tests::test_morris_code_not_empty ... ok
test tests::test_letter_a ... ok
test tests::test_morris_code_table_length_55 ... ok
test tests::test_number_42 ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Timed output on the same line

I want to output the Morris Code on the same line with a specific interval of time between characters. For example, the time for dots are 250 ms and dashes are 500 ms. Why? Because eventually this will be the sound duration. I used the use `std::time::Duration` and `use std::thread` to allow me to set the “sleep” intervals. 

I want to loop through the Morris Code and printing a character, waiting a set amount of time, then print another character on the same line until the message is complete. My first attempt did not turn out as expected. 

```rust
    for found in  morris.chars(){
        if found == '-' {
            thread::sleep(Duration::from_millis(dash_time));
        } else {
            thread::sleep(Duration::from_millis(dot_time));
        }
        print!(“{found}”);
    }
```

This waits for the amount an amount of time the prints the entire message on one line. No delay between characters. If I used `println!` I get the delay between characters but each character is on it’s own line! Bummer. Why? Turns out the print! macro locks the standard output on each call. So, if you call `print!` in a loop the output is buffered until the loop completes. The solution is to use `io::stdout().lock()`. This piece of documentation helped me out a lot. [Macro std::print](https://doc.rust-lang.org/std/macro.print.html)

Instead of using the print macro we lock the stdout, print the character then flush the io buffer.

```rust
        write!(lock, "{found}").unwrap();
        io::stdout().flush().unwrap();
```

Works great.