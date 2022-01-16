# morris code rust project

It's just for fun.

- Convert to run on rpi3/4.
- Blink the light
- Use speaker to "do" the sound

**Sentence mode:**
- -s
- Type a sentence. Enter causes morris code print out.
- Prints two lines. Top line is the message, second line is morris code
- Each word is lined up with corresponding code
- End of line is CR/LF

**Word mode:**
- -w
- Type a word then "enter"
- Prints two lines. Top line is the word, second line is morris code
- Each character is lined up with corresponding code

*Character mode:*
- -c
- Interactive. As soon as the key is pressed the code is printed.
- CR/LF exits program