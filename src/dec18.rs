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
      assert!(super::find_innermost("123".to_string(),&super::evaluate)==123);
      assert!(super::find_innermost("1 + (2 * 3) + (4 * (5 + 6))".to_string(),&super::evaluate)==51);
      assert!(super::find_innermost("2 * 3 + (4 * 5)".to_string(),&super::evaluate)==26);
      assert!(super::find_innermost("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),&super::evaluate)==437);
      assert!(super::find_innermost("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),&super::evaluate)==12240);
      assert!(super::find_innermost("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),&super::evaluate)==13632);
      assert!(26335==super::solve_part1("./inputs/dec18-test.txt".to_string()));

      assert!(super::evaluate_part2("1 + 2 * 3 + 4 * 5 + 6".to_string())==231);
      assert!(super::find_innermost("1 + (2 * 3) + (4 * (5 + 6))".to_string(),&super::evaluate_part2)==51);
      assert!(super::find_innermost("2 * 3 + (4 * 5)".to_string(),&super::evaluate_part2)==46);
      assert!(super::find_innermost("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),&super::evaluate_part2)==1445);
      assert!(super::find_innermost("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),&super::evaluate_part2)==669060);
      assert!(super::find_innermost("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),&super::evaluate_part2)==23340);
      super::solve_part2("./inputs/dec18-test.txt".to_string());
  }  
}

use std::fs;
use regex::Regex;

lazy_static! {
  static ref INNER_REGEX: Regex = Regex::new(r"^(.*)\(([0-9 *+]+)\)(.*)$").unwrap();
  static ref ADD_REGEX: Regex = Regex::new(r"^(.*?)(\d+)\s+\+\s+(\d+)(.*)$").unwrap();
  static ref MULTIPLY_REGEX: Regex = Regex::new(r"^(.*?)(\d+)\s+\*\s+(\d+)(.*)$").unwrap();
}

pub fn find_innermost (expression : String, eval: &dyn Fn(String) -> i64 ) -> i64 {

  if expression.find("(") == None  {
    return eval(expression);
  }
    
  if let Some(inner) = INNER_REGEX.captures(&expression) {
      let r = eval(inner[2].to_string());
      let new_string = format!("{} {} {}", &inner[1].trim(), r, &inner[3].trim());
      return find_innermost(new_string, &eval);
  }

  panic!("should enver get here");
}


pub fn evaluate(expression : String) -> i64 {
  let mut retval = 0;
  if expression.find("(") != None || expression.find(")") != None {
    panic!("trying to evaluate [{}] with paranethsis", expression);
  }
  let mut first = true;
  let mut operator = "";
  let elements = expression.split_ascii_whitespace().collect::<Vec<&str>>();

  for element in elements {
    if first {
      retval = element.parse::<i64>().unwrap();
      first= false;
    } else if element == "+" || element == "*" {
      operator = element;
    } else {
      if operator == "+" {
        retval = retval + element.trim().parse::<i64>().unwrap();
      } else {
        retval = retval * element.trim().parse::<i64>().unwrap();
      }
    }

  }
  retval
}

pub fn evaluate_part2(expression : String) -> i64 {
  if expression.find("(") != None || expression.find(")") != None {
    panic!("trying to evaluate with paranethsis");
  }

  if let Ok(n) = expression.trim().parse::<i64>() {
    return n;
  }


  let new_string = expression;
  let next_string;

  let result;

  if let Some(inner) = ADD_REGEX.captures(&new_string) {
    result = inner[2].parse::<i64>().unwrap() + inner[3].parse::<i64>().unwrap();
    next_string = format!("{} {} {}", inner[1].to_string(), result, inner[4].to_string());
    return evaluate_part2(next_string);
  } else if let Some(inner) = MULTIPLY_REGEX.captures(&new_string) {
    result = inner[2].parse::<i64>().unwrap() * inner[3].parse::<i64>().unwrap();
    next_string = format!("{} {} {}", inner[1].to_string(), result, inner[4].to_string());
    return evaluate_part2(next_string);
  }
  panic!("shouldn't get here");
}

pub fn solve_part1(filename : String) -> u64 {
  // println!("Day 18 part 1");
  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();

  for line in lines {
    // print!("{}=", &line);
    let e = find_innermost(line, &evaluate) as u64;
    // println!("{}", e);
    retval += e;
  }
  retval
}

pub fn solve_part2(filename : String) -> u64 {
  // println!("Day 18 part 2");

  let mut retval = 0;
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
  for line in lines {
    // print!("{}=", &line);
    let e = find_innermost(line,&evaluate_part2) as u64;
    // println!("{}", e);
    retval += e;
  }

  retval
}
