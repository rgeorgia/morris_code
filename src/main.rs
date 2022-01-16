

/*
Mode:
    Sentence mode:
        -s
        Type a sentence. Enter causes morris code print out.
        Prints two lines. Top line is the message, second line is morris code
        Each word is lined up with corresponding code
        End of line is CR/LF
    Word mode:
        -w
        Type a word then "enter"
        Prints two lines. Top line is the word, second line is morris code
        Each character is lined up with corresponding code
    Character mode:
        -c
        Interactive. As soon as the key is pressed the code is printed.
        CR/LF exits program
*/



fn main() {
    let morris_table = morris_lib::morris_code_table();

    // for (key, value) in &morris_table {
    //     println!("{} : {}", key, value)
    // }
    println!("Quote is {}", morris_table.get("\"").unwrap()) ;
}
