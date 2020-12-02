use std::fs;

#[derive(Debug)]
pub struct Password {
  rule: String,
  character: String,
  password_text: String,
}

fn get_password_from_line(line: String) -> Password {
  let password_text = line.split(":").collect::<Vec<&str>>()[1].trim();
  let rule_line = line.split(":").collect::<Vec<&str>>()[0].trim();
  let rule = rule_line.split(" ").collect::<Vec<&str>>()[0].trim();
  let character = rule_line.split(" ").collect::<Vec<&str>>()[1].trim();

  Password {rule: rule.to_string(), character: character.to_string(), password_text: password_text.to_string()}

}


pub fn read_input(filename: String) -> Vec<Password> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");
  let pwds : Vec<Password> = contents.lines().map(|line| get_password_from_line(line.to_string()) ).collect();
  pwds
}

pub fn solve_part1(pwds: &Vec<Password>) {
  let mut good_count = 0;

  for p in pwds {
    let splits = p.password_text.split(&p.character);
    let count = splits.collect::<Vec<&str>>().len() as i32 - 1;
    
    let lo_hi = p.rule.split("-").collect::<Vec<&str>>();

    let lo = lo_hi[0].parse::<i32>().unwrap();
    let hi = lo_hi[1].parse::<i32>().unwrap();

    if lo <= count && count <= hi {
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
    let lo_hi = p.rule.split("-").collect::<Vec<&str>>();

    let lo = lo_hi[0].parse::<i32>().unwrap() as usize-1;
    let hi = lo_hi[1].parse::<i32>().unwrap() as usize-1;

    let slice1 = p.character == p.password_text[lo..lo+1];
    let slice2 = p.character == p.password_text[hi..hi+1];

    if ((slice1) || (slice2)) && !((slice1) && (slice2)){
      good_count = good_count + 1;
      println!("{} of {} char and between {} and {}", p.password_text, p.character, lo, hi);

    }
  }
  println!("Good passwords {}", good_count);
}
 