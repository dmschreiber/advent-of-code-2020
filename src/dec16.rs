
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
    let args : Vec<&str> = line.split(":").collect();
    let field_name = args[0];
    let range_nums : Vec<String>= args[1].split("or").map(|n| n.to_string()).collect();

    let rn2 : Vec<i32>= range_nums[0].split("-").map(|n| n.trim().parse::<i32>().unwrap()).collect();
    let rn3 : Vec<i32>= range_nums[1].split("-").map(|n| n.trim().parse::<i32>().unwrap()).collect();
    // println!("{:?} {:?}", rn2, rn3);
    let r = Range { name: field_name.to_string(), lo : rn2[0], hi : rn2[1], lo2 : rn3[0], hi2 : rn3[1] };
    ranges.push(r);

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

  let mut fail = 0;
  let mut valid_lines = <Vec<Vec<i32>>>::new();

  for line in other_ticket_lines {
    let numbers : Vec<i32>;
    if line != "nearby tickets:" {
      // println!("{}", line);

      numbers = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
      let mut in_range = false;

      for n in &numbers {
        in_range = false;
        for range in &ranges {
            if (n >= &range.lo) && (n <= &range.hi) || (n >= &range.lo2) && (n <= &range.hi2) {
              // println!("fail range {} {} , {} {} val {}", lo, hi, lo2, hi2, n);
              in_range = true;
            }
          
        }
        if !in_range {
          fail += n;
          break;
        } 
      }
      if in_range {
          valid_lines.push(numbers.clone());
      }
    }

  }
  println!("Day 16 part 1 Ticket scanning error rate {}", fail);
  // valid_lines with arrany of number
  let max_nums = valid_lines[0].len();
  let mut mapped_range = std::collections::HashMap::new();

  loop {
    if mapped_range.len() >= ranges.len() {
      break;
    } 
    
    let mut index = 0;
    loop {
      if index >= max_nums {
        break;
      }
      // println!("Trying the {} position", index);
      let mut which_range_valid = None;
      let mut valid_range_count = 0;
      for (which_range,range) in ranges.iter().enumerate() {
        // skip the range if it's already been mapped
        if mapped_range.get(&which_range) == None {
          let mut valid = true;

          for n in &valid_lines {
            if !((n[index] >= range.lo) && (n[index] <= range.hi) || (n[index] >= range.lo2) && (n[index] <= range.hi2)) {
              valid = false;
              break;
            }
          }
          
          // all numbers in this column meet the range rules
          if valid {
            which_range_valid = Some(which_range);
            valid_range_count += 1;
          } 
          
        }
      }

      // when exactly one range fits a column; store it
      if valid_range_count == 1 {
        if let Some(i) = which_range_valid {      
          mapped_range.insert(i,index);

        }
      }
    
      index += 1;
    }
  }

  let mut retval : i64 = 1;
  for k in mapped_range.keys() {
    let range = &ranges[*k];
    if range.name.find("departure") != None {
      // println!("range {:?} at col {} my number is {}", ranges[*k as usize], mapped_range.get(k).unwrap(),my_numbers[*mapped_range.get(k).unwrap() as usize]);
      retval = retval * my_numbers[*mapped_range.get(k).unwrap() as usize] as i64;
    }
  }

  retval
}