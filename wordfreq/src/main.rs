// word frequency

// Reads text from the standard input and 
// writes the frequency of different words to the standard output.

// INPUT

// The input format could be anything:

//     Hello world
//     www333
//     github.com/rust
//     *&^%$@!#@!

// Any non-alphabetic character will be regarded as noise and will not be counted:
//     ///233
//     ++--

// The input terminates with either end-of-file or a line "999".

// OUTPUT

// The program computes the 


use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
fn main() {
    word_count(&read_input(stdin()))
    //write_output(stdout());
}
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
// #[cfg(test)]
// mod read_lines_test {
//     use super::read_lines;
//     use std::io::Cursor;

//     #[test]
//     fn reads_three_numbers() {
//         assert_eq!("3.\n4.\n5.\n", read_lines("3.\n4.\n5.\n"));
//     }

//     // fn assert_read(expected: &[f64], input: &str) {
//     //     let mock_read = Cursor::new(input);
//     //     let measurements = read_measurements(mock_read);
//     //     assert_eq!(expected.to_owned(), measurements);
//     // }

// }

pub fn word_count(input: &str)  {
    let mut map: HashMap<String, u32> = HashMap::new();
    let lower = input.to_lowercase();
    let slice: &str = lower.as_ref();
    for word in slice.split(|c: char| !c.is_alphabetic()).filter(|s| !s.is_empty()) {
        *map.entry(word.to_string()).or_insert(0)+=1;
    }
    
    let mut count_vec: Vec<(&String, &u32)> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    //println!("Most frequent character in text: {}: {}", count_vec[0].0, count_vec[0].1);
    for i in 0..count_vec.len() {
        let (key,value) = count_vec[i].clone();
        println!("{}: {}", key, value);
    }
    
    // let result : &Vec<(String,u32)>;
    // Vec::from[count_vec];
    // return result;
    
    
}

// pub fn write_output<W: Write>(mut writer: W) {
//   //let m: HashMap<String, u32> = word_count(&read_input(stdin()));
//   let m: Vec<(String,u32)> =  word_count(&read_input(stdin())); 
//   if m.is_empty() {
//       write!(writer, "No word found.\n").unwrap();
//   } 
//   else {
//     //   for (key,value) in m.iter() {
//     //       write!(writer,"{}: {}\n", key,value).unwrap();
//     //   }
//     for x in m.len() {
//         write!(writer,"{} : {}", &m[x].0, &m[x].1).unwrap();
//     }
    
//   }
// }
