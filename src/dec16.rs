
#[cfg(test)]
mod tests {

  #[test]
  pub fn dec16_prod() {
      let lines: Vec<String> = include_str!("../inputs/dec16.txt").lines().map(|s| (&*s).to_string() ).collect();
      assert!(239727793813==super::solve(&lines));
  }
  #[test]
  pub fn dec16_test() {
      let lines: Vec<String> = include_str!("../inputs/dec16-test.txt").lines().map(|s| (&*s).to_string() ).collect();
      assert!(1==super::solve(&lines));
  }
}

#[derive(Debug)]
enum Field {
  Range(String,i32,i32,i32,i32),
}

pub fn solve(lines : &Vec<String>) -> i64 {

  let mut ranges = vec![];

  for line in lines {
    if line == "your ticket:" || line.len() == 0 {
      break;
    }
    // println!("{}", line);

    let args : Vec<&str> = line.split(":").collect();
    let field_name = args[0];
    let range_nums : Vec<String>= args[1].split("or").map(|n| n.to_string()).collect();

    let rn2 : Vec<i32>= range_nums[0].split("-").map(|n| n.trim().parse::<i32>().unwrap()).collect();
    let rn3 : Vec<i32>= range_nums[1].split("-").map(|n| n.trim().parse::<i32>().unwrap()).collect();
    // println!("{:?} {:?}", rn2, rn3);
    let r = Field::Range(field_name.to_string(), rn2[0], rn2[1], rn3[0], rn3[1]);
    ranges.push(r);

  }

  // GET MY NUMBERS
  let mut my_numbers = vec![];
  let mut my_tickets = false;
  for line in lines {
    if line == "your ticket:" {
      my_tickets = true;
    } else if my_tickets {
      // println!("my numbers {}", line);
      my_numbers = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
      break;
    }
  }

  let mut nearby_tickets = false;
  let mut fail = 0;
  let mut valid_lines = <Vec<Vec<i32>>>::new();

  for line in lines {
    let numbers : Vec<i32>;
    if line == "nearby tickets:" {
      nearby_tickets = true;
    }
    else if nearby_tickets {
      // println!("{}", line);

      numbers = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
      let mut in_range = false;

      for n in &numbers {
        in_range = false;
        for range in &ranges {
          let Field::Range(_name, lo, hi, lo2, hi2) =range; 
            if (n >= lo) && (n <= hi) || (n >= lo2) && (n <= hi2) {
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
    } else {
      // println!("Found {}", mapped_range.len());
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
        if mapped_range.get(&which_range) == None {
          let mut valid = true;
          let Field::Range(_name, lo, hi, lo2, hi2) =range; 
          // println!("Trying range {} with {}-{} and {}-{}", name, lo, hi, lo2, hi2);
          for n in &valid_lines {
            if (n[index] >= *lo) && (n[index] <= *hi) || (n[index] >= *lo2) && (n[index] <= *hi2) {
              // println!("num {} range {} is valid for ranges {}-{} and {}-{}", n[index], name, lo, hi, lo2, hi2);
            } else {
              // println!("num {} range {} {}-{} {} {} is invalid for index {}", n[index], name, lo, hi, lo2, hi2, index);
              valid = false;
              break;
            }
          }
          if valid {
            // println!("===>range {} is valid for index {}", name, index);
            which_range_valid = Some(which_range);
            valid_range_count += 1;
          } else {
            // println!("range {} is invalid for index {}", name, index);
          }
        }
      }
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
    let Field::Range(name, _lo, _hi, _lo2, _hi2) = &ranges[*k];
    if name.find("departure") != None {
      // println!("range {:?} at col {} my number is {}", ranges[*k as usize], mapped_range.get(k).unwrap(),my_numbers[*mapped_range.get(k).unwrap() as usize]);
      retval = retval * my_numbers[*mapped_range.get(k).unwrap() as usize] as i64;
    }
  }

  retval
}