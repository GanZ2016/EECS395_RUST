use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
fn main() {
    //let text = read_lines();
    write_output(stdout());
}

pub fn read_lines() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
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

pub fn check_word_count(s: &str, pairs: Vec<(&str, u32)>) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<String, u32> = word_count(s);
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k.to_string()).unwrap_or(0)), (k, v));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&String,&u32)>>(), vec!());
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

// #[cfg(test)]
mod check_word_count_test{
    use super::check_word_count;
    #[test]
    fn test_count_one_word() {
        check_word_count("word", vec![("word", 1)]);
    }
    #[test]
    fn test_count_multiple_occurrences() {
        check_word_count(
            "one fish two fish red fish blue fish",
            vec![("one", 1),
                ("fish", 4),
                ("two", 1),
                ("red", 1),
                ("blue", 1)]);
    }

    #[test]

    fn test_ignore_punctuation() {
        check_word_count(
            "car : carpet as java : javascript!!&@$%^&",
            vec![("car", 1),
                ("carpet", 1),
                ("as", 1),
                ("java", 1),
                ("javascript", 1)]);
    }

    #[test]

    fn test_include_numbers() {
        check_word_count(
            "testing, 1, 2 testing",
            vec![("testing", 2),
                ("1", 1),
                ("2", 1)]);
    }

    #[test]

    fn test_normalize_case() {
        check_word_count(
            "go Go GO Stop stop",
            vec![("go", 3),
                ("stop", 2)]);
    }
}
