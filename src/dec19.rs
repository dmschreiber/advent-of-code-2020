#[cfg(test)]
mod tests {

  #[test]
  pub fn dec19_prod() {
      let (rules,messages_lines) = super::load_rules("./inputs/dec19.txt".to_string());
      println!("Day 19 part 1 is {}", super::solve_part1(&rules, &messages_lines));
      assert!(226==super::solve_part1(&rules, &messages_lines));
      // println!("Day 18 part 2 is {}", super::solve_part2("./inputs/dec18.txt".to_string()));
      // assert!(20394514442037==super::solve_part2("./inputs/dec18.txt".to_string()));
  }
  #[test]
  pub fn dec19_test() {
    // assert!(super::evaluate("1 + 2 * 3 + 4 * 5 + 6".to_string())==71);
    let (rules,messages_lines) = super::load_rules("./inputs/dec19-test.txt".to_string());
    assert!(2==super::solve_part1(&rules, &messages_lines));
      
    assert!(super::does_match(&rules,4,Some("ab".to_string())) != None);
    assert!(super::does_match(&rules,5,Some("ba".to_string())) != None);
    println!("{:?}",super::does_match(&rules,0,Some("ababbb".to_string())));
    assert!(super::does_match(&rules,0,Some("ababbb".to_string())) == Some("".to_string()));
    assert!(super::does_match(&rules,0,Some("abbbab".to_string())) == Some("".to_string()));
    assert!(super::does_match(&rules,0,Some("bababa".to_string())) == None);
    assert!(super::does_match(&rules,0,Some("aaabbb".to_string())) == None);
    assert!(super::does_match(&rules,0,Some("aaaabbb".to_string())) != Some("".to_string()));

      // super::solve_part2("./inputs/dec18-test.txt".to_string());
  }  
}

use std::fs;
use regex::Regex;

lazy_static! {
  static ref RULE_OR_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+)$").unwrap();
  static ref RULE_SIMPLE_OR_REGEX: Regex = Regex::new(r"^(\d+): (\d+) \| (\d+)$").unwrap();
  static ref RULE_LITERAL_REGEX: Regex = Regex::new(r#"^(\d+): .([ab]).$"#).unwrap();
  static ref RULE_BASIC1_REGEX: Regex = Regex::new(r"^(\d+): (\d+)$").unwrap();
  static ref RULE_BASIC2_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+)$").unwrap();
  static ref RULE_BASIC3_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) (\d+)$").unwrap();
}

#[derive(Debug)]
pub enum Arg {
  Basic1(u32),
  Basic2(u32,u32),
  Basic3(u32,u32,u32),
  Literal(String),  
}

#[derive(Debug)]
pub enum Rule {
  Or(Arg,Arg),
  Value(Arg),
  // SimpleOr(u32,u32),
  // Or(u32,u32,u32,u32),
  // Basic1(u32),
  // Basic2(u32,u32),
  // Basic3(u32,u32,u32),
  // Literal(String),  
}

fn do_basic1(rules : &std::collections::HashMap<u32,Rule>, rule_number : u32, line : Option<String>) -> Option<String> {
  if let Some(s1) = does_match(rules, rule_number, line) {
    Some(s1)
  } else {
    None
  }
}

fn do_basic2(rules : &std::collections::HashMap<u32,Rule>, r1 : u32, r2 : u32, line : Option<String>) -> Option<String> {
  if let Some(s1) = does_match(rules, r1, line) {
    if let Some(s2) = does_match(rules, r2, Some(s1)) {
        Some(s2)
      } else {
        None
      }
    } else { None }
}
fn do_basic3(rules : &std::collections::HashMap<u32,Rule>, r1 : u32, r2 : u32, r3 : u32, line : Option<String>) -> Option<String> {
  if let Some(s1) = does_match(rules, r1, line) {
    if let Some(s2) = does_match(rules, r2, Some(s1)) {
      if let Some(s3) = does_match(rules, r3, Some(s2)) {
        Some(s3)
      } else {
        None
      }
    } else {
      None
    }
  } else { None }
}

fn does_match (rules : &std::collections::HashMap<u32,Rule>, rule_number : u32, line : Option<String>) -> Option<String> {

  if line == None { return None; }
  let my_string = line.unwrap().clone();
  // println!("PROCESSING RULE {} against string {}", rule_number, my_string);
  let rule = rules.get(&rule_number).unwrap();
  let retval = 
  match rule 
  {
    Rule::Value(value) => 
    { 
        match value 
        {
          Arg::Literal(l) => 
                    {
                      if my_string[..1] == *l {
                      // if let Some(at_pos) = my_string.find(l)  { 
                        let new_string = format!("{}",&my_string[1..]).to_string();

                        Some(new_string)
                      } else {
                        // println!("did not find {} in {}", l,my_string);
                        None 
                      }
                    }
          Arg::Basic1(r1) => { do_basic1(rules, *r1, Some(my_string)) }      
          Arg::Basic2(r1,r2) => { do_basic2(rules, *r1, *r2, Some(my_string)) }
          Arg::Basic3(r1,r2,r3) => { do_basic3(rules, *r1, *r2, *r3, Some(my_string)) }
        }
    }
    Rule::Or(p1,p2) => 
    {
      match p1 
      {
        Arg::Basic2(p1_a1,p1_a2) => 
        {
          match p2 
          {
            Arg::Basic1(p2_a1) => { panic!("or with 2 and 1")}
            Arg::Basic2(p2_a1,p2_a2) => 
            { 
              let p1 = do_basic2(rules,*p1_a1,*p1_a2, Some(my_string.clone())); 
              if p1 == None {
                do_basic2(rules,*p2_a1,*p2_a2,Some(my_string.clone()))
              } else {
                p1
              }
            }
            _ => { panic!("Or with more than two args")}
          }
        }
        Arg::Basic1(p1_a1) => {
          match p2 {
            Arg::Basic1(p2_a1) => {
              if let Some(s1) = do_basic1(rules, *p1_a1, Some(my_string.clone())) {
                Some(s1)
              } else {
                do_basic1(rules, *p2_a1, Some(my_string.clone()))
              }
            }
            _ => {panic!("Or with 1 and more than one arg")}
          }
        }
        _ => { panic!("Or with more than two args")}
      }
    }
    _ => { None }                              
    };

  retval
}

pub fn load_rules(filename : String) -> (std::collections::HashMap::<u32,Rule>, Vec<String>) {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let section : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let rules_lines = section[0].lines();
  let messages_lines = section[1].lines().map(|s| (&*s).to_string()).collect();
  let mut rules = std::collections::HashMap::<u32,Rule>::new();

  for line in rules_lines {
    println!("RULES LINE {}", line);
    if let Some(args) = RULE_OR_REGEX.captures(line) {
      // println!("OR");
      let rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p1_a2 = args[3].parse::<u32>().unwrap();
      let p2_a1 = args[4].parse::<u32>().unwrap();
      let p2_a2 = args[5].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic2(p1_a1,p1_a2),Arg::Basic2(p2_a1,p2_a2)));
      // rules.insert(rule_number,Rule::Or(p1_a1,p1_a2,p2_a1,p2_a2));

      // println!("{} {} {} {} {}", rule_number, p1_a1, p1_a2, p2_a1, p2_a2);
    } else if let Some(args) = RULE_SIMPLE_OR_REGEX.captures(line) {
      // println!("SIMPLE OR");
      let rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p2_a1 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic1(p1_a1),Arg::Basic1(p2_a1)));
      // rules.insert(rule_number,Rule::SimpleOr(p1_a1,p2_a1));

    } else if let Some(args) = RULE_LITERAL_REGEX.captures(line) {
      let rule_number = args[1].parse::<u32>().unwrap();
      let literal = args[2].to_string();
      rules.insert(rule_number, Rule::Value(Arg::Literal(literal)));
      // println!("LITERAL {} {:?}", rule_number, rules.get(&rule_number).unwrap());
    } else if let Some(args) = RULE_BASIC3_REGEX.captures(line) {
      let rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      let l3 = args[4].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic3(l1, l2, l3)));
      // println!("BASIC 3 {} {} {} {}", rule_number, &l1, &l2, &l3);
    } else if let Some(args) = RULE_BASIC2_REGEX.captures(line) {
      let rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic2(l1, l2)));
      // println!("BASIC 2 {} {} {}", rule_number, &l1, &l2);
    } else if let Some(args) = RULE_BASIC1_REGEX.captures(line) {
      let rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic1(l1)));
      // println!("BASIC 1 {} {}", rule_number, &l1);

    } else {
      panic!("unexpected rule format! ({})",line);
    }
    // println!("{:?}", rules);

  }
  (rules,messages_lines)
}

pub fn solve_part1(rules : &std::collections::HashMap::<u32,Rule>, messages_lines : &Vec<String>) -> u64 {
  // println!("Day 18 part 1");
  let mut retval = 0;

  for line in messages_lines {
    let m = does_match(&rules, 0,Some(line.to_string()));
    // println!("{} - {:?}", line, m);
    if m == Some("".to_string()) {
      retval += 1;
    }
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
    // let e = find_innermost(line,&evaluate_part2) as u64;
    // println!("{}", e);
    // retval += e;
  }

  retval
}

pub fn solve(filename : String) {
  let (rules,messages_lines) = load_rules(filename.to_string());
  println!("Day 19 part 1 is {}", solve_part1(&rules, &messages_lines));
  assert!(226==solve_part1(&rules, &messages_lines));

}