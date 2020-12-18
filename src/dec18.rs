#[cfg(test)]
mod tests {

  #[test]
  pub fn dec18_prod() {
      println!("Day 18 part 1 is {}", super::solve_part1("./inputs/dec18.txt".to_string()));
      assert!(650217205854==super::solve_part1("./inputs/dec18.txt".to_string()));
      println!("Day 18 part 2 is {}", super::solve_part2("./inputs/dec18.txt".to_string()));
      assert!(20394514442037==super::solve_part2("./inputs/dec18.txt".to_string()));
  }
  #[test]
  pub fn dec18_test() {
      assert!(super::evaluate("1 + 2 * 3 + 4 * 5 + 6".to_string())==71);
      assert!(super::find_innermost("123".to_string(),false)==123);
      assert!(super::find_innermost("1 + (2 * 3) + (4 * (5 + 6))".to_string(),false)==51);
      assert!(super::find_innermost("2 * 3 + (4 * 5)".to_string(),false)==26);
      assert!(super::find_innermost("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),false)==437);
      assert!(super::find_innermost("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),false)==12240);
      assert!(super::find_innermost("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),false)==13632);
      assert!(26335==super::solve_part1("./inputs/dec18-test.txt".to_string()));

      assert!(super::evaluate_part2("1 + 2 * 3 + 4 * 5 + 6".to_string())==231);
      assert!(super::find_innermost("1 + (2 * 3) + (4 * (5 + 6))".to_string(),true)==51);
      assert!(super::find_innermost("2 * 3 + (4 * 5)".to_string(),true)==46);
      assert!(super::find_innermost("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),true)==1445);
      assert!(super::find_innermost("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),true)==669060);
      assert!(super::find_innermost("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),true)==23340);
      super::solve_part2("./inputs/dec18-test.txt".to_string());
  }
  
}

use std::fs;

pub fn find_innermost (expression : String, part2 : bool ) -> i64 {
  let mut retval;
  let mut new_string = expression;
  let mut keep_going = true;

  let mut offset = 0;
  while keep_going {
    if let Some(start) = new_string[offset..].find("(")  {
      if let Some(next_close) = new_string[offset+start+1..].find(")") {
        match new_string[offset+start+1..].find("(") {
          Some(next_open) if next_close+start < next_open+start => {
            retval = match part2 {
              true => evaluate_part2(new_string[offset+start+1..=offset+next_close+start].to_string()),
              false => evaluate(new_string[offset+start+1..=offset+next_close+start].to_string())
            };

            if offset+start > 0 {
              new_string = new_string[..=offset+start-1].to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];
            } else {
              new_string = "".to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];

            }
              // println!("now i have {}", new_string);
            offset = 0;
          }
          None => {
            retval = match part2 {
              true =>  evaluate_part2(new_string[offset+start+1..=offset+next_close+start].to_string()),
              false => evaluate(new_string[offset+start+1..=offset+next_close+start].to_string())
            };

            if offset+start > 0 {
              new_string = new_string[..=offset+start-1].to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];
            } else {
              new_string = "".to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];

            }
            offset = 0;
          }
          _ => {
            offset = offset+1;
            if offset > new_string.len() {
              break;
            }
          }
        }
      }
    } else {
      keep_going = false;
    }
  }

  retval = match part2 {
    true => evaluate_part2(new_string),
    false => evaluate(new_string)
  };

  retval
}

pub fn evaluate(expression : String) -> i64 {
  let mut retval = 0;
  if expression.find("(") != None || expression.find(")") != None {
    panic!("trying to evaluate with paranethsis");
  }
  let mut first = true;
  let mut operator = "";
  let elements = expression.split(" ").collect::<Vec<&str>>();

  for element in elements {
    if first {
      retval = element.parse::<i64>().unwrap();
      first= false;
    } else if element == "+" || element == "*" {
      operator = element;
    } else {
      if operator == "+" {
        retval = retval + element.parse::<i64>().unwrap();
      } else {
        retval = retval * element.parse::<i64>().unwrap();
      }
    }

  }
  retval
}

pub fn evaluate_part2(expression : String) -> i64 {
  let mut retval = 0;

  if expression.find("(") != None || expression.find(")") != None {
    panic!("trying to evaluate with paranethsis");
  }

  if let Ok(n) = expression.trim().parse::<i64>() {
    // println!("Just a number {}", n);
    return n;
  }

  let mut operator = "";

  let mut new_string = expression;
  let mut next_string = String::from("");

  let has_addition = new_string.find("+") != None;
  let elements = new_string.clone().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

  let mut result = 0;
  let mut last_arg = 0;

  if has_addition {
    for (i,element) in elements.iter().enumerate() {
      if let Ok(n) = element.parse::<i64>() {
        if operator == "+" {
          result = last_arg + n;
          if i == 2 {
            next_string = format!("{} {}",result, &elements[i+1..].join(" ").to_string()).to_string();
          } else {
            next_string = format!("{} {} {}",elements[..i-2].join(" "), result, &elements[i+1..].join(" ")).to_string();
          }
          // println!("new string is {}", next_string);
          return evaluate_part2(next_string);
        }
        last_arg = n;
      } else if *element == "+" || *element == "*" {
        operator = element;
      } 
    }
  } else {
    for (i,element) in elements.iter().enumerate() {
      if let Ok(n) = element.parse::<i64>() {
        if operator == "*" {
          result = last_arg * n;
          if i == 2 {
            next_string = format!("{} {}",result, &elements[i+1..].join(" ").to_string()).to_string();
          } else {
            next_string = format!("{} {} {}",elements[..i-2].join(" "), result, &elements[i+1..].join(" ")).to_string();
          }
          // println!("new string is {}", next_string);
          return evaluate_part2(next_string);
        }
        last_arg = n;
      } else if *element == "+" || *element == "*" {
        operator = element;
      } 
    }

  }
  retval
}

pub fn solve_part1(filename : String) -> u64 {
  println!("Day 18 part 1");
  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  for line in lines {
    print!("{}=", &line);
    let e = find_innermost(line,false) as u64;
    println!("{}", e);
    retval += e;
  }
  retval
}

pub fn solve_part2(filename : String) -> u64 {
  println!("Day 18 part 2");

  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
  for line in lines {
    print!("{}=", &line);
    let e = find_innermost(line,true) as u64;
    println!("{}", e);
    retval += e;
  }

  retval
}