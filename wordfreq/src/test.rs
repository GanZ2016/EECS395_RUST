use std::io::prelude::*;                                                           
use std::io;                                                                       

fn main() {
    let c = 'a';
    
   if (c.is_alphabetic()) {
       println!("true");
       io::stdout().flush().ok().expect("Could not flush stdout");
   } 
   else {
       println!("error");
       io::stdout().flush().ok().expect("Could not flush stdout");
   }
}
