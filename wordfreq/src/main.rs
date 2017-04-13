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
    Explanation:In our program, apostrophe(') is considered as a splitter, so "what's" is regared as two seperate words "what" and "s".
                For the similar situation like "he's", "I'm" or "Let's", apostrophes are all regarded as splitters.

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
use std::collections::HashMap;
fn main() {
    write_output(stdout(),word_sort(&word_count(&read_input(stdin()))));
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
    return input;
}
#[cfg(test)]
mod read_input_test {
    use super::read_input;
    use std::io::Cursor;

    #[test]
    fn reads_anything() {
        assert_read("\n2.3.4.5.6", "2.3.4.5.6");
    }
    #[test]
    fn read_nothing() {
        assert_read("", "");
    }
    fn assert_read(expected: &str, input: &str) {
    let reader = Cursor::new(input);
    let read = read_input(reader);
    assert_eq!(expected.to_owned(), read);
}
}

pub fn word_count(input: &str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let lower = input.to_lowercase(); //turn the input string into lowercase
    let slice: &str = lower.as_ref();
    //split the input string and search the word in hashmap, get value and plus 1
    for word in slice.split(|c: char| !c.is_alphabetic()).filter(|s| !s.is_empty()) {
        *map.entry(word.to_string()).or_insert(0)+=1;
    }

    map
}

#[cfg(test)]
mod test_word_count{
    use super::word_count;
    use std::collections::HashMap;
    #[test]
    fn count_some_words() {
        let input = "Hello world world! ";
        let map: HashMap<String, usize>  = word_count(input);
        assert_eq!(map[&"hello".to_string()], 1);
        assert_eq!(map[&"world".to_string()], 2);
    }
}

fn word_sort(map: &HashMap<String, usize>) -> Vec<(&String, &usize)> {
    //sort the hashmap and put the result in a vector
    let mut count_vec: Vec<(&String, &usize)> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    //print elements in the vector line by line
    count_vec
}

#[cfg(test)]
mod test_word_sort{
    use super::word_sort;
    use std::collections::HashMap;
    #[test]
    fn word_sort_test() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("Hello".to_string(),2);
        map.insert("world".to_string(),3);
        map.insert("test".to_string(),4);
        let count = word_sort(&map);
        assert_eq!(count[0].0,"test");
        assert_eq!(count[0].1, &(4 as usize));
        assert_eq!(count[1].0,"world");
        assert_eq!(count[1].1, &(3 as usize));
        assert_eq!(count[2].0,"Hello");
        assert_eq!(count[2].1, &(2 as usize));
    }  
}



pub fn write_output<W: Write>(mut writer: W, r:Vec<(&String,&usize)>) {
  if r.is_empty() {
      write!(writer, "No word found.\n").unwrap();
  } 
  else {
      for i in 0..r.len() {
        let (key,value) = r[i].clone();
        write!(writer,"{}: {}\n", key,value).unwrap();   
        }
    }
}

#[cfg(test)]
mod test_write_output{
	use super::write_output;
	use std::io::Cursor;
    use super::word_sort;
    use std::collections::HashMap;
	#[test]
	fn output_result(){
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(),1);
        map.insert("world".to_string(),3);
        map.insert("test".to_string(),4);
        let count = word_sort(&map);
		assert_write("test: 4\nworld: 3\nhello: 1\n", count);
		}

    fn assert_write(expected: &str, result: Vec<(&String,&usize)>) {
        let mut writer = Cursor::new(vec![]);
        write_output(&mut writer, result);
        assert_eq!(expected.as_bytes(), &*writer.into_inner());
    }
}