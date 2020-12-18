#[cfg(test)]
mod tests {

  #[test]
  pub fn dec18_prod() {
      println!("Day 18 part 1 is {}", super::solve_part1("./inputs/dec18.txt".to_string()));
      // assert!(10267078750!=super::solve_part1("./inputs/dec18.txt".to_string()));
      // super::solve_part2("./inputs/dec18.txt".to_string());
  }
  #[test]
  pub fn dec18_test() {
      assert!(super::evaluate("1 + 2 * 3 + 4 * 5 + 6".to_string())==71);
      assert!(super::find_innermost("123".to_string())==123);
      assert!(super::find_innermost("1 + (2 * 3) + (4 * (5 + 6))".to_string())==51);
      assert!(super::find_innermost("2 * 3 + (4 * 5)".to_string())==26);
      assert!(super::find_innermost("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string())==437);
      assert!(super::find_innermost("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string())==12240);
      assert!(super::find_innermost("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())==13632);
      assert!(26335==super::solve_part1("./inputs/dec18-test.txt".to_string()));

      // assert!(848==super::solve_part2("./inputs/dec18-test.txt".to_string()));
  }
  
}

use std::fs;

pub fn find_innermost (expression : String ) -> i64 {
  let mut retval = 0;
  let mut new_string = expression;
  let mut keep_going = true;

  let mut offset = 0;
  while keep_going {
    // println!("==> {}", &new_string[offset..]);
    if let Some(start) = new_string[offset..].find("(")  {
      // println!("found open {}", start);
      if let Some(next_close) = new_string[offset+start+1..].find(")") {
        // println!("found close {}", start+next_close);
        match new_string[offset+start+1..].find("(") {
          Some(next_open) if next_close+start < next_open+start => {
            // println!("found next open {} in {}", start+next_open, &new_string[offset+start..]);
            // println!("found {}", new_string[offset+start+1..=offset+next_close+start].to_string());
            retval = evaluate(new_string[offset+start+1..=offset+next_close+start].to_string());

            if offset+start > 0 {
              new_string = new_string[..=offset+start-1].to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];
            } else {
              new_string = "".to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];

            }
              // println!("now i have {}", new_string);
            offset = 0;
          }
          None => {
            retval = evaluate(new_string[offset+start+1..=offset+next_close+start].to_string());
            // println!("{}",retval);
            // println!("beginning part offset {}, start {}", offset,start);
            // println!("ending part {}", &new_string[offset+start+next_close+2..]);
            if offset+start > 0 {
              new_string = new_string[..=offset+start-1].to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];
            } else {
              new_string = "".to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];

            }
            // println!("now i have {}", new_string);
            offset = 0;
          }
          _ => {
            // println!("new offset is {}", offset);
            offset = offset+1;
            if offset > new_string.len() {
              break;
            }
          }
        }
        // if let Some(next_open) = new_string[offset+start+1..].find("(") {
        //   println!("found next open {} in {}", start+next_open, &new_string[offset+start..]);
        //   if next_close+start < next_open+start || new_string[offset+start..].find("(") == None {
        //     println!("found {}", new_string[offset+start+1..=offset+next_close+start].to_string());
        //     retval = evaluate(new_string[offset+start+1..=offset+next_close+start].to_string());
        //     println!("{}",retval);
        //     new_string = new_string[..=offset+start-1].to_string() + &format!("{}",retval) + &new_string[offset+start+next_close+2..];
        //     println!("now i have {}", new_string);
        //     offset = 0;
        //   } else {
        //     println!("new offset is {}", offset);
        //     offset = offset+1;
        //     if offset > new_string.len() {
        //       break;
        //     }
        //   }
        // }
      }
    } else {
      keep_going = false;
    }
  }
  retval = evaluate(new_string);
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
pub fn solve_part1(filename : String) -> u64 {
  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  for line in lines {
    print!("{}=", &line);
    let e = find_innermost(line) as u64;
    println!("{}", e);
    retval += e;
  }
  retval
}

// pub fn solve_part2(filename : String) -> i64 {
//   let mut retval = 0;
//   let contents = fs::read_to_string(filename)
//   .expect("Something went wrong reading the file");

//   let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
//   retval
// }
