/*word frequency

Reads text from the standard input and 
writes the frequency of different words to the standard output.

INPUT

The input format could be anything:

    Hello world
    www333
    github.com/rust
    *&^%$@!#@!

Any non-alphabetic character will be regarded as noise and will not be counted:
    ///233
    ++--

The input terminates with either end-of-file or a line "999".

OUTPUT

The program computes the frequencies of input and show the words with their frequencies in descending order

ASSUMPTIONS

-   There is no limitation of the input, so it could be characters, numbers, or any legal special symbols

-   The program only counts words, and a single character like 'x' is regarded as a word. Here are some special cases:
    
    INPUT:  1a2b3c
            999
    OUTPUT: b: 1
            a: 1
            c: 1
    
    INPUT:  What's your name
            999
    OUTPUT: s: 1
            what: 1
            name: 1
            your: 1

    INPUT:  999
    OUTPUT: No word found.

    INPUT:  a b  c   /a   ?b
            c-- ..
            999
    OUTPUT: b: 2
            a: 2
            c: 2

-   The terminator is a line of text "999" or the end of file, not a line of text that
    when interpreted is merely the number 999.0.*/


use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
fn main() {
    word_count(&read_input(stdin()))
}
//standard input and store the result in a string
pub fn read_input<R: Read>(reader: R) -> String {
    let mut input = String::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
        let space = "";
        let res = [space, &line].join("\n");
        input.push_str(&res);  
    }
    input
}
#[cfg(test)]
mod read_lines_test {
    use super::read_input;
    use std::io::Cursor;

    #[test]
    fn reads_three_numbers() {
        assert_eq!("2.3.4.5.6", read_input("2.3.4.5.6"));
    }

    fn assert_read(expected: &[f64], input: &str) {
        let mock_read = Cursor::new(input);
        let test = read_input(mock_read);
        assert_eq!(expected.to_owned(), test);
    }

}

pub fn word_count(input: &str)  {
    let mut map: HashMap<String, u32> = HashMap::new();
    let lower = input.to_lowercase(); //turn the input string into lowercase
    let slice: &str = lower.as_ref();
    //split the input string and search the word in hashmap, get value and plus 1
    for word in slice.split(|c: char| !c.is_alphabetic()).filter(|s| !s.is_empty()) {
        *map.entry(word.to_string()).or_insert(0)+=1;
    }
    //sort the hashmap and put the result in a vector
    let mut count_vec: Vec<(&String, &u32)> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    //print elements in the vector line by line
    write_output(stdout(),count_vec);
    
}

pub fn write_output<W: Write>(mut writer: W, r:Vec<(&String,&u32)>) {
  if r.is_empty() {
      write!(writer, "No word found.\n").unwrap();
  } 
  else {
      for i in 0..r.len() {
        let (key,value) = r[i].clone();
        write!(writer,"{}: {}\n", key,value).unwrap();   
  }
}
