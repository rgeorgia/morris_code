# Writing the morris code program with Rust

## First steps

I know I need a "table" that translates ASCII to Morris Code, so I went with a HashMap, where the key is the ASCII character and the value is the dashes and dots. 

Before branching out into the world of crates and separate files I put everything in one file to start with. The first iteration just loads the map and prints out two hard coded values.

The `morris_code_table` is a Vec of String types. Why? Because neither the value nor the keys will need to be changed. 

Added four tests, because that exactly what should be done.