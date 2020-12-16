
#[cfg(test)]
mod tests {

  #[test]
  pub fn dec16_prod() {
      // let lines: Vec<String> = include_str!("../inputs/dec16.txt").lines().map(|s| (&*s).to_string() ).collect();
      assert!(239727793813==super::solve("./inputs/dec16.txt".to_string()));
  }
  #[test]
  pub fn dec16_test() {
      // let lines: Vec<String> = include_str!("../inputs/dec16-test.txt").lines().map(|s| (&*s).to_string() ).collect();
      assert!(1==super::solve("./inputs/dec16-test.txt".to_string()));
  }
}

use std::fs;
use regex::Regex;
lazy_static! {
  static ref RULE_REGEX: Regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

#[derive(Debug)]
pub struct Range {
  name :String,
  lo : i32,
  hi : i32,
  lo2 : i32,
  hi2 : i32,
}

pub fn solve(filename : String) -> i64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let section : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let rules_lines = section[0].lines();
  let your_ticket_lines = section[1].lines();
  let other_ticket_lines = section[2].lines();
  
  let mut ranges = vec![];

  for line in rules_lines {
    if let Some(item) = RULE_REGEX.captures(line) {
      ranges.push ( Range { name: item[1].to_string(), 
        lo : item[2].parse().unwrap(), 
        hi : item[3].parse().unwrap(), 
        lo2 : item[4].parse().unwrap(), 
        hi2 : item[5].parse().unwrap() });
    }
  }

  // GET MY NUMBERS
  let mut my_numbers = vec![];

  for line in your_ticket_lines {
    if line != "your ticket:" {
      // println!("my numbers {}", line);
      my_numbers = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
      break;
    }
  }

  // PROCESS OTHER TICKETS
  let mut fail = 0;
  let mut valid_lines = <Vec<Vec<i32>>>::new();

  for line in other_ticket_lines {
    let numbers : Vec<i32>;
    if line != "nearby tickets:" {
      numbers = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();

      for (position,n) in numbers.iter().enumerate() {
        // if it it fits in none of the ranges
        if ! ranges
            .iter()
            .fold(false, |acc,range| acc || (n >= &range.lo) && (n <= &range.hi) || (n >= &range.lo2) && (n <= &range.hi2) ) {
          fail += n;
          break;

        } else if position == numbers.len() - 1 {
          // last number means it's a valid ticket
          valid_lines.push(numbers.clone());
        }
      }
    }
  }

  println!("Day 16 part 1 Ticket scanning error rate {}", fail);
  
  // valid_lines with arrany of number
  let max_nums = valid_lines[0].len();
  let mut mapped_range = std::collections::HashMap::new();

  loop { // Keep looking until you've mapped them all
    if mapped_range.len() >= ranges.len() {
      break;
    } 
    
    for index in 0..max_nums {
      let mut which_range_valid = None;

      for (which_range,range) in ranges.iter().enumerate() {
        // skip the range if it's already been mapped
        if mapped_range.get(&which_range) == None {

          let valid = valid_lines
              .iter()
              .fold(true, |acc,n| acc && ((n[index] >= range.lo) && (n[index] <= range.hi) || (n[index] >= range.lo2) && (n[index] <= range.hi2) ));
          
          // all numbers in this column meet the range rules
          if valid && which_range_valid == None {
            which_range_valid = Some(which_range);
          } else if valid { // second one so stop
            which_range_valid = None;
            break;
          } 
        }
      }

      if let Some(i) = which_range_valid {      
        mapped_range.insert(i,index);
      }

    }
  }

  let mut retval : i64 = 1;
  for k in mapped_range.keys() {
    let range = &ranges[*k];
    if range.name.find("departure") != None {
      retval = retval * my_numbers[*mapped_range.get(k).unwrap() as usize] as i64;
    }
  }

  retval
}