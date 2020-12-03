use std::fs;

#[derive(Debug)]
pub struct Password {
  rule: String,
  hi: i32,
  lo: i32,
  character: String,
  password_text: String,
}

fn get_password_from_line(line: String) -> Password {
  let password_text = line.split(":").collect::<Vec<&str>>()[1].trim();
  let rule_line = line.split(":").collect::<Vec<&str>>()[0].trim();
  let rule = rule_line.split(" ").collect::<Vec<&str>>()[0].trim();
  let character = rule_line.split(" ").collect::<Vec<&str>>()[1].trim();
  let lo_hi = rule.split("-").collect::<Vec<&str>>();

  let lo = lo_hi[0].parse::<i32>().unwrap();
  let hi = lo_hi[1].parse::<i32>().unwrap();

  Password {rule: rule.to_string(), character: character.to_string(), password_text: password_text.to_string(), hi: hi, lo: lo, }

}


pub fn read_input(filename: String) -> Vec<Password> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  let pwds : Vec<Password> = contents.lines().map(|line| get_password_from_line(line.to_string()) ).collect();
  pwds
}

fn count_chars(my_string: &str, my_char: u8) -> i32 {
  let mut count = 0;
  for c in my_string.as_bytes() {
    
    if my_char == *c {
      count = count + 1;
    }
  }
  count
}

pub fn solve_part1(pwds: &Vec<Password>) {
  let mut good_count = 0;

  for p in pwds {
    let count = count_chars(&p.password_text, p.character.as_bytes()[0]);    

    if p.lo <= count && count <= p.hi {
      // good
      good_count = good_count + 1;
    } else {
      // println!("{} has {} of {} char and between {} and {}", p.password_text, count, p.character, lo, hi);
    }
  }
  println!("Good passwords {}", good_count);
}
pub fn solve_part2(pwds: &Vec<Password>) {
  let mut good_count = 0;

  for p in pwds {

    let lo = p.lo as usize - 1;
    let hi = p.hi as usize - 1;

    let slice1 = p.character == p.password_text[lo..lo+1];
    let slice2 = p.character == p.password_text[hi..hi+1];

    if ((slice1) || (slice2)) && !((slice1) && (slice2)){
      good_count = good_count + 1;
      // println!("{} of {} char and between {} and {}", p.password_text, p.character, lo, hi);

    }
  }
  println!("Good passwords {}", good_count);
}
 