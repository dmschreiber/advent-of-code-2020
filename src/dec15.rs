use std::collections::HashMap;
use std::time::{Instant};
// use regex::Regex;

#[cfg(test)]
mod tests {
  // #[derive(Debug)]
  // enum Thing {
  //   Argument(u32),
  //   Message(String),
  // }

  #[test]
  pub fn solve_prod() {
      let lines: Vec<String> = include_str!("../inputs/dec15.txt").lines().map(|s| (&*s).to_string() ).collect();
      super::solve(lines,2020);
  }

  #[test]
  pub fn solve_test() {
    let lines: Vec<String> = include_str!("../inputs/dec15-test.txt").lines().map(|s| (&*s).to_string() ).collect();
    super::solve(lines,2020);
  }
}


pub fn solve(lines : Vec<String>, position : usize) {
      // let mut things = <Vec<Thing>>::new();
      let mut start = Instant::now();
      let line = &lines[0];
      let args : Vec<u32> = line.split(",").map(|s| (&*s).parse::<u32>().unwrap() ).collect();
      println!("INPUT LINE {}",line);
      
      let mut history = <HashMap<u32,Vec<usize>>>::new();

      // solve
      let mut index = 0;
      let mut last_number = 0;
      let mut num;
      loop {

          if index < args.len() {
            num = args[index];
          } else if let Some(pos_array) = history.get(&last_number) {
            if pos_array.len() < 2 {
              num = 0;
            } else {
              num = (pos_array[0] - pos_array[1])as u32;
            }
          } else {
            panic!("Shouldn't get here");
          }
          
          if let Some(pos_array) = history.get_mut(&num) { // None or Some(value)
            pos_array.insert(0,index);
            if pos_array.len() > 2 {
              pos_array.pop();
            }
          } else {
            // let mut positions = <Vec<usize>>::new();
            // positions.insert(0,index);
            // history.insert(num,positions);
            history.insert(num,vec![index]); 
          }
          // 3, [1,2,3]
          // 5, [5,6,7]

          index += 1;
          last_number = num;

          if index == position {
            println!("num: {}", num);
            break;
          } else {
              if index % 5000000 == 0 {
                println!("checking: {} at {} ({:?})", num, index, start.elapsed());
                start = Instant::now();
              }        
          }
      }
}

