use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
fn main() {
    //let text = read_lines();
    write_output(stdout());
}
pub fn read_input<R: Read>(reader: R) -> String {
    let mut input = String::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        // if let Ok(f) = line.parse(){
        //     input.push_str(f);
        // }
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

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let lower = input.to_lowercase();
    let slice: &str = lower.as_ref();
    for word in slice.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

pub fn write_output<W: Write>(mut writer: W) {
  let mut m: HashMap<String, u32> = word_count(&read_input(stdin()));
  if m.is_empty() {
      write!(writer, "No measurements provided.\n").unwrap();
  } 
  else {
      for (key,value) in m.iter() {
          write!(writer,"{} {}\n", key, value).unwrap();
      }
      //write!(writer,"{}\n",m::iter().key).unwrap();
      //write!(writer, "Below count:   {}\n", r.below).unwrap();
  }
}
