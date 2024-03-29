pub mod morris_code {
    use std::collections::HashMap;

    // https://morsecode.world/international/morse2.html

    pub fn morris_code_table() -> HashMap<String, String> {
        let mut morris_table: HashMap<String, String> = HashMap::new();
        morris_table.insert(String::from("A"), String::from(".-"));
        morris_table.insert(String::from("B"), String::from("-..."));
        morris_table.insert(String::from("C"), String::from("-.-."));
        morris_table.insert(String::from("D"), String::from("."));
        morris_table.insert(String::from("E"), String::from("."));
        morris_table.insert(String::from("F"), String::from("..-."));
        morris_table.insert(String::from("G"), String::from("--."));
        morris_table.insert(String::from("H"), String::from("...."));
        morris_table.insert(String::from("I"), String::from(".."));
        morris_table.insert(String::from("J"), String::from(".---"));
        morris_table.insert(String::from("K"), String::from("-.-"));
        morris_table.insert(String::from("L"), String::from(".-.."));
        morris_table.insert(String::from("M"), String::from("--"));
        morris_table.insert(String::from("N"), String::from("-."));
        morris_table.insert(String::from("O"), String::from("---"));
        morris_table.insert(String::from("P"), String::from(".--."));
        morris_table.insert(String::from("Q"), String::from("--.-"));
        morris_table.insert(String::from("R"), String::from(".-."));
        morris_table.insert(String::from("S"), String::from("..."));
        morris_table.insert(String::from("T"), String::from("-"));
        morris_table.insert(String::from("U"), String::from("..-"));
        morris_table.insert(String::from("V"), String::from("...-"));
        morris_table.insert(String::from("W"), String::from(".--"));
        morris_table.insert(String::from("X"), String::from("-..-"));
        morris_table.insert(String::from("Y"), String::from("-.--"));
        morris_table.insert(String::from("Z"), String::from("--.."));
        morris_table.insert(String::from("0"), String::from("-----"));
        morris_table.insert(String::from("1"), String::from(".----"));
        morris_table.insert(String::from("2"), String::from("..---"));
        morris_table.insert(String::from("3"), String::from("...--"));
        morris_table.insert(String::from("4"), String::from("....-"));
        morris_table.insert(String::from("5"), String::from("....."));
        morris_table.insert(String::from("6"), String::from("-...."));
        morris_table.insert(String::from("7"), String::from("--..."));
        morris_table.insert(String::from("8"), String::from("---.."));
        morris_table.insert(String::from("9"), String::from("----."));
        morris_table.insert(String::from("Error"), String::from("........"));
        morris_table.insert(String::from("&"), String::from(".-..."));
        morris_table.insert(String::from("'"), String::from(".----."));
        morris_table.insert(String::from("@"), String::from(".--.-."));
        morris_table.insert(String::from(")"), String::from("-.--.-"));
        morris_table.insert(String::from("("), String::from("-.--."));
        morris_table.insert(String::from(":"), String::from("---..."));
        morris_table.insert(String::from(","), String::from("--..--"));
        morris_table.insert(String::from("="), String::from("-...-"));
        morris_table.insert(String::from("!"), String::from("-.-.--"));
        morris_table.insert(String::from("."), String::from(".-.-.-"));
        morris_table.insert(String::from("-"), String::from("-....-"));
        morris_table.insert(String::from("×"), String::from("-..-"));
        morris_table.insert(String::from("%"), String::from("----- -..-. -----"));
        morris_table.insert(String::from("+"), String::from(".-.-."));
        morris_table.insert(String::from("\""), String::from(".-..-."));
        morris_table.insert(String::from("?"), String::from("..--.."));
        morris_table.insert(String::from("/"), String::from("-..-."));
        morris_table.insert(String::from(" "), String::from(" "));

        morris_table
    }
}
