#[cfg(test)]
mod tests {

  #[test]
  pub fn dec19_prod() {
      let (rules,messages_lines) = super::load_rules("./inputs/dec19.txt".to_string());
      println!("Day 19 part 1 is {}", super::solve_part1(&rules, &messages_lines));
      assert!(226==super::solve_part1(&rules, &messages_lines));

      let val = super::solve_part2(&rules, &messages_lines);
      println!("part two prod 2 {}",val);
      assert!(val==355);

  }
  #[test]
  pub fn dec19_test() {
    let (rules,messages_lines) = super::load_rules("./inputs/dec19-test.txt".to_string());
    assert!(2==super::solve_part1(&rules, &messages_lines));

    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("ababbb".to_string()),None) == Some("".to_string()));
    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("abbbab".to_string()),None) == Some("".to_string()));
    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("bababa".to_string()),None) == None);
    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("aaabbb".to_string()),None) == None);
    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("aaaabbb".to_string()),None) != Some("".to_string()));

    let (rules,messages_lines) = super::load_rules("./inputs/dec19-test3.txt".to_string());
    println!("Test 3 {:?}",super::solve_part1(&rules, &messages_lines));

    let (mut rules,messages_lines) = super::load_rules("./inputs/dec19-test2.txt".to_string());
    assert!(3==super::solve_part1(&rules, &messages_lines));

    rules.remove(&8);
    rules.remove(&11);

    let all_lines = vec!["abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
                        "bbabbbbaabaabba",
                        "babbbbaabbbbbabbbbbbaabaaabaaa",
                        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                        "bbbbbbbaaaabbbbaaabbabaaa",
                        "bbbababbbbaaaaaaaabbababaaababaabab",
                        "ababaaaaaabaaab",
                        "ababaaaaabbbaba",
                        "baabbaaaabbaaaababbaababb",
                        "abbbbabbbbaaaababbbbbbaaaababb",
                        "aaaaabbaabaaaaababaa",
                        "aaaabbaaaabbaaa",
                        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                        "babaaabbbaaabaababbaabababaaab",
                        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"];
                        
    let matched_lines = vec![
                        "bbabbbbaabaabba",
                        "babbbbaabbbbbabbbbbbaabaaabaaa",
                        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                        "bbbbbbbaaaabbbbaaabbabaaa",
                        "bbbababbbbaaaaaaaabbababaaababaabab",
                        "ababaaaaaabaaab",
                        "ababaaaaabbbaba",
                        "baabbaaaabbaaaababbaababb",
                        "abbbbabbbbaaaababbbbbbaaaababb",
                        "aaaaabbaabaaaaababaa",
                        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];               
    
    for l in all_lines {
      let matched = super::rule_zero(&rules, &l.to_string()); 
      if matched && !matched_lines.contains(&l) {
        println!(" [[{} shouldn't match but does]]", &l);
      } else if !matched && matched_lines.contains(&l) {
        println!(" [[{} doesn't match but should]]", &l);
        let result = super::does_match(&rules,rules.get(&42).unwrap().clone(),Some(l.to_string()),None);
        println!("{:?}",result);
        let result = super::does_match(&rules,rules.get(&42).unwrap().clone(),result,None);
        println!("{:?}",result);
        let result = super::does_match(&rules,rules.get(&42).unwrap().clone(),result,None);
        println!("{:?}",result);
        let result = super::does_match(&rules,rules.get(&42).unwrap().clone(),result,None);
        println!("{:?}",result);
        let result = super::does_match(&rules,rules.get(&31).unwrap().clone(),result,None);
        println!("{:?}",result);
        break;
      }
    }
    let val = super::solve_part2(&rules, &messages_lines);
    println!("part two test should wrong (3 not 12) - {}",val);

    assert!(12==val);

  }  
}

use std::fs;
use regex::Regex;

lazy_static! {
  static ref RULE_OR_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+)$").unwrap();
  static ref RULE_SIMPLE_OR_REGEX: Regex = Regex::new(r"^(\d+): (\d+) \| (\d+)$").unwrap();
  static ref RULE_OR12_REGEX: Regex = Regex::new(r"^(\d+): (\d+) \| (\d+) (\d+)$").unwrap();
  static ref RULE_OR23_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+) (\d+)$").unwrap();

  static ref RULE_LITERAL_REGEX: Regex = Regex::new(r#"^(\d+): .([ab]).$"#).unwrap();
  static ref RULE_BASIC1_REGEX: Regex = Regex::new(r"^(\d+): (\d+)$").unwrap();
  
  static ref RULE_BASIC2_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+)$").unwrap();
  static ref RULE_BASIC3_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) (\d+)$").unwrap();
}

#[derive(Debug,Clone,PartialEq)]
pub enum Arg {
  Basic1(u32),
  Basic2(u32,u32),
  Basic3(u32,u32,u32),
  Literal(String),  
}

#[derive(Debug,Clone,PartialEq)]
pub enum Rule {
  Or(Arg,Arg),
  Value(Arg),
}

fn do_basic1(rules : &std::collections::HashMap<u32,Rule>, rule_number : u32, line : Option<String>, level : Option<String>) -> Option<String> {
  if let Some(r) = rules.get(&rule_number) {
    if let Some(s1) = does_match(rules, r.clone(), line,level) {
      Some(s1)
    } else {
      None
    }
  } else {
    panic!("Can't find rule {}", rule_number);
  }
}

fn do_basic2(rules : &std::collections::HashMap<u32,Rule>, r1 : u32, r2 : u32, line : Option<String>, level : Option<String>) -> Option<String> {
  let r1_o = rules.get(&r1).unwrap();
  let r2_o = rules.get(&r2).unwrap();

  if let Some(s1) = does_match(rules, r1_o.clone(), line.clone(),level.clone()) 
  {
    if let Some(s2) = does_match(rules, r2_o.clone(), Some(s1),level.clone()) 
    {
        Some(s2)
    } else if let Rule::Or(a1,a2) = r1_o {
      if std::mem::discriminant(a1) != std::mem::discriminant(a2)
      {
        if let Some(s1) = does_match(rules, Rule::Value(a2.clone()), line, level.clone()) {
          if let Some(s2) = does_match(rules, r2_o.clone(), Some(s1),level.clone()) 
          {
              Some(s2)
          } else {None }
        } else { None }
      } else 
      { None }
    } else { None } // second rule
  } else { None } // first rule
}
fn do_basic3(rules : &std::collections::HashMap<u32,Rule>, r1 : u32, r2 : u32, r3 : u32, line : Option<String>, level : Option<String>) -> Option<String> {
  let r1_o = rules.get(&r1).unwrap();
  let r2_o = rules.get(&r2).unwrap();
  let r3_o = rules.get(&r3).unwrap();

  if let Some(s1) = does_match(rules, r1_o.clone(), line,level.clone()) {
    if let Some(s2) = does_match(rules, r2_o.clone(), Some(s1),level.clone()) {
      if let Some(s3) = does_match(rules, r3_o.clone(), Some(s2),level.clone()) {
        Some(s3)
      } else {
        None
      }
    } else {
      None
    }
  } else { None }
}

fn does_match (rules : &std::collections::HashMap<u32,Rule>, rule : Rule, line : Option<String>, level : Option<String>) -> Option<String> {
  if line == None { return None; }
  // if line == Some("".to_string()) {return None; }

  let my_string = line.unwrap().clone();
  // let rule = rules.get(&rule_number).unwrap();
  
  let level_format;
  if level == None {
    level_format = "".to_string();
  } else {
    level_format = level.unwrap();
  }
  // println!("{} PROCESSING RULE against string {} - {:?}", level_format, &my_string, rule);

  let retval = 
  match rule 
  {
    Rule::Value(value) => 
    { 
        match value 
        {
          Arg::Literal(l) => 
          { if my_string.len() == 0 { None }
            else if *l == my_string[..1] { 
              // println!("{} returning Some {}",level_format,my_string);
              Some(format!("{}",&my_string[1..]).to_string()) 
            } else {
              None
            }
          }
          Arg::Basic1(r1) => { do_basic1(rules, r1, Some(my_string),Some(format!("{}>",level_format))) }      
          Arg::Basic2(r1,r2) => { do_basic2(rules, r1, r2, Some(my_string),Some(format!("{}>",level_format))) }
          Arg::Basic3(r1,r2,r3) => { do_basic3(rules, r1, r2, r3, Some(my_string),Some(format!("{}>",level_format))) }
        }
    }
    Rule::Or(p1,p2) => 
    {
      match p1 
      {
        Arg::Basic1(p1_a1) => {
          match p2 {
            Arg::Basic1(p2_a1) => {
              if let Some(s1) = do_basic1(rules, p1_a1, Some(my_string.clone()), Some(format!("{}>",level_format))) {
                Some(s1)
              } else {
                do_basic1(rules, p2_a1, Some(my_string.clone()),Some(format!("{}>",level_format)))
              }
            }
            Arg::Basic2(p2_a1,p2_a2) => {
              if let Some(s1) = do_basic1(rules, p1_a1, Some(my_string.clone()),Some(format!("{}>",level_format))) {
                Some(s1)
              } else {
                do_basic2(rules, p2_a1, p2_a2, Some(my_string.clone()),Some(format!("{}>",level_format)))
              }
            }
            _ => {panic!("Or with 1 and more than one arg")}
          }
        }        
        Arg::Basic2(p1_a1,p1_a2) => 
        {
          match p2 
          {
            Arg::Basic1(_p2_a1) => { panic!("or with 2 and 1")}
            Arg::Basic2(p2_a1,p2_a2) => 
            { 
              let p1 = do_basic2(rules,p1_a1,p1_a2, Some(my_string.clone()),Some(format!("{}>",level_format))); 
              if p1 == None {
                do_basic2(rules,p2_a1,p2_a2,Some(my_string.clone()),Some(format!("{}>",level_format)))
              } else {
                p1
              }
            }
            Arg::Basic3(p2_a1,p2_a2,p2_a3) => {
              let p1 = do_basic2(rules, p1_a1, p1_a2, Some(my_string.clone()),Some(format!("{}>",level_format)));
              if p1 == None {
                do_basic3(rules, p2_a1, p2_a2, p2_a3, Some(my_string.clone()),Some(format!("{}>",level_format)))
              } else {
                p1
              }
            }
            _ => { panic!("Or with more than two args")}
          }
        }

        _ => { panic!("Or with more than two args")}
      }
    }
    };
  if retval != None {
    // println!("{} RULE RESULT {:?}", level_format, retval);
  }
  retval
}

pub fn load_rules(filename : String) -> (std::collections::HashMap::<u32,Rule>, Vec<String>) {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let section : Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string()).collect();
  let rules_lines = section[0].lines();
  let messages_lines = section[1].lines().map(|s| (&*s).to_string()).collect();
  let mut rules = std::collections::HashMap::<u32,Rule>::new();
  let mut rule_number;

  for line in rules_lines {
    // print!("RULES LINE {}", line);
    if let Some(args) = RULE_OR_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p1_a2 = args[3].parse::<u32>().unwrap();
      let p2_a1 = args[4].parse::<u32>().unwrap();
      let p2_a2 = args[5].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic2(p1_a1,p1_a2),Arg::Basic2(p2_a1,p2_a2)));
    } else if let Some(args) = RULE_SIMPLE_OR_REGEX.captures(line) {
      // println!("SIMPLE OR");
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p2_a1 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic1(p1_a1),Arg::Basic1(p2_a1)));
      // rules.insert(rule_number,Rule::SimpleOr(p1_a1,p2_a1));

    } else if let Some(args) = RULE_OR12_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p2_a1 = args[3].parse::<u32>().unwrap();
      let p2_a2 = args[4].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic1(p1_a1),Arg::Basic2(p2_a1,p2_a2)));
    } else if let Some(args) = RULE_OR23_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p1_a2 = args[3].parse::<u32>().unwrap();
      let p2_a1 = args[4].parse::<u32>().unwrap();
      let p2_a2 = args[5].parse::<u32>().unwrap();
      let p2_a3 = args[6].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(Arg::Basic2(p1_a1,p1_a2),Arg::Basic3(p2_a1,p2_a2,p2_a3)));

    } else if let Some(args) = RULE_LITERAL_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let literal = args[2].to_string();
      rules.insert(rule_number, Rule::Value(Arg::Literal(literal)));
      // println!("LITERAL {} {:?}", rule_number, rules.get(&rule_number).unwrap());
    } else if let Some(args) = RULE_BASIC3_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      let l3 = args[4].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic3(l1, l2, l3)));
      // println!("BASIC 3 {} {} {} {}", rule_number, &l1, &l2, &l3);
    } else if let Some(args) = RULE_BASIC2_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic2(l1, l2)));
      // println!("BASIC 2 {} {} {}", rule_number, &l1, &l2);
    } else if let Some(args) = RULE_BASIC1_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Value(Arg::Basic1(l1)));
      // println!("BASIC 1 {} {}", rule_number, &l1);

    } else {
      panic!("unexpected rule format! ({})",line);
    }
    // println!("{:?}", rules.get(&rule_number).unwrap());

  }
  (rules,messages_lines)
}

pub fn solve_part1(rules : &std::collections::HashMap::<u32,Rule>, messages_lines : &Vec<String>) -> u64 {
  let mut retval = 0;

  for line in messages_lines {
    let m = does_match(&rules, rules.get(&0).unwrap().clone(),Some(line.to_string()),None);
    if m == Some("".to_string()) {
      // println!("{} - {:?}", line.len(), m);
      retval += 1;
    }
  }
  retval
}


fn rule_zero(rules : &std::collections::HashMap::<u32,Rule>, messages_line : &String) -> bool {
  let mut result;

  // Option 1 is just 42 42 31
  result = does_match(&rules, Rule::Value(Arg::Basic3(42,42,31)),Some(messages_line.to_string()),None);
  if result == Some("".to_string()) {
    return true;
  }

  // Option 2 is 42(n+2) 31
  // 42 8 42 31 -> 42 (42 | 42 8) 42 31 -> 42 (42 | 42 (42 | 42 8)) 42 31
  // 42 42 42 31, 42 42 42 42 31, ... 42 (n+2) times then 31 where n >= 1
  // IN TEST Rule 42 matches 5 chars; Rule 31 matches 5 chars
  // IN PROD Rule 42 and Rule 31 match 8 chars
  // println!("LINE {}", messages_line);

  for n in 1..=9 {
    // Do Rule 42 n+2 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }
    // Do Rule 31 one times
    result = do_basic1(&rules, 31, result, None);

    if result == Some("".to_string()) {
      return true;
    }  
  }

  // remaining option 3
  // 42 42 11 31 -> 42 42 (42 31 | 42 11 31) 31 -> 42 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31 
  //                                            -> 42 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31) 31
  // 42 42 42 31 31, 42 42 42 42 31 31 31, 42 42 42 42 42 31 31 31 31, 
  //                                       42x(n+2) and 31x(n+1) where n>=1 and n <= 5
  // 42 42 42 42 42 42 31 31 31 31 31 is the longest that could match
  // for all n>= 2 match 42 n+1 times and 31 n times (lengths 25, 35, 45, 55, etc chars IN TEST)
  // Option 3 summary
  // 42(n+2) 31(n+1) n>=1 n<=5

  for n in 1..=6 {
    // Do Rule 42 n+2 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }
    // Do Rule 31 n+1 times
    result = do_basic1(&rules, 31, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 31, result, None);
    }

    if result == Some("".to_string()) {
      return true;
    }  
  }

  // remaining option 4
  // ******* NOTES ********
  // 42 8 42 11 31 -> 42 (42 | 42 8) 42 (42 31 | 42 11 31) 31
  //                  42 (42 | 42 (42 | 42 8)) 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31

  // option 4.1         opion 4.2                      option 4.3                            option 4.4
  // 42 42 42 42 31 31, 42 42 8 42 42 31 31,           42 42 42 42 11 31 31,                
  //                --> 42 42 (42 | 42 8) 42 42 31 31, 42 42 42 42 (42 31 | 42 11 31) 31 31, 
  //                    42(n+3) 31 31 n>=1             42(n+4) 31(n+2) n>= 1                 
  //  
  // option 4.4
  // 42 42 8 42 42 11 31 31
  // 42 42 (42 | 42 8) 42 42 (42 31 | 42 11 31) 31 31

  // option 4.4.1                option 4.4.2                  option 4.4.3                   
  // 42 42 42 42 42 42 31 31 31, 42 42 42 8 42 42 42 31 31 31, 42 42 42 42 42 42 11 31 31 31, 
  //                             42(n+6) 31 31 31 n>=1         42(n+6) 31(n+3) n>=1

  // option 4.4.4
  // 42 42 42 8 42 42 42 11 31 31 31
  // 42 42 42 (42 | 42 8) 42 42 42 (42 31 | 42 11 31) 31 31 31
  // 42x8 31x4, 42(n+8) 31x4, 42x8 31(n+4), 42x4 8 42x4 11 31x4 <-- too long
  // ******* END NOTES ********
  // Option 4 Summary (9 options)
  // 
  // 42x4 31x2
  result = do_basic3(&rules, 42, 42, 42, Some(messages_line.to_string()), None);
  result = do_basic1(&rules, 42, result, None);
  result = do_basic2(&rules, 31, 31, result, None);

  if result == Some("".to_string()) {
    return true;
  }  


  // 42(n+3) 31x2 n>=1
  for n in 1..=8 {
    // Do Rule 42 n+3 times
    result = do_basic3(&rules, 42, 42, 42, Some(messages_line.to_string()), None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }
    // Do Rule 31 two times
    result = do_basic2(&rules, 31, 31, result, None);

    if result == Some("".to_string()) {
      return true;
    }  
  }

  // 42(n+4) 31(n+2) n>= 1
  for n in 1..=5 {
    // Do Rule 42 n+4 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    result = do_basic2(&rules, 42, 42, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }
    // Do Rule 31 n+2 times
    result = do_basic2(&rules, 31, 31, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 31, result, None);
    }

    if result == Some("".to_string()) {
      return true;
    }  
  }

  // 42x6 31x3
  // Do Rule 42 6 times
  result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
  result = do_basic2(&rules, 42, 42, result, None);
  result = do_basic2(&rules, 42, 42, result, None);

  // Do Rule 31 3 times
  result = do_basic2(&rules, 31, 31, result, None);
  result = do_basic1(&rules, 31, result, None);

  if result == Some("".to_string()) {
    return true;
  }  

  // 42(n+6) 31x3 n>=1
  for n in 1..=5 {
    // Do Rule 42 n+6 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }
    // Do Rule 31 3 times
  // Do Rule 31 3 times
    result = do_basic2(&rules, 31, 31, result, None);
    result = do_basic1(&rules, 31, result, None);

    if result == Some("".to_string()) {
      return true;
    }  
  }

  // 42(n+6) 31(n+3) n>=1
  for n in 1..=5 {
    // Do Rule 42 n+6 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }

    // Do Rule 31 n+3 times
    result = do_basic3(&rules, 31, 31, 31, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 31, result, None);
    }

    if result == Some("".to_string()) {
      return true;
    }  
  }  
  // 42x8 31x4
  // Do Rule 42 8 times
  result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
  result = do_basic2(&rules, 42, 42, result, None);
  result = do_basic2(&rules, 42, 42, result, None);
  result = do_basic2(&rules, 42, 42, result, None);

  // Do Rule 31 4 times
  result = do_basic2(&rules, 31, 31, result, None);
  result = do_basic2(&rules, 31, 31, result, None);

  if result == Some("".to_string()) {
    return true;
  }  

  // 42(n+8) 31x4 n>=1
  for n in 1..=5 {
    // Do Rule 42 n+8 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 42, result, None);
    }

    // Do Rule 31 4 times
    result = do_basic2(&rules, 31, 31, result, None);
    result = do_basic2(&rules, 31, 31, result, None);

    if result == Some("".to_string()) {
      return true;
    }  
  }  

  // 42x8 31(n+4) n>=1
  for n in 1..=5 {
    // Do Rule 42 n+8 times
    result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);
    result = do_basic2(&rules, 42, 42, result, None);

    // Do Rule 31 n+4 times
    result = do_basic2(&rules, 31, 31, result, None);
    result = do_basic2(&rules, 31, 31, result, None);
    for _i in 0..n {
      result = do_basic1(&rules, 31, result, None);
    }

    if result == Some("".to_string()) {
      return true;
    }  
  }  

  return false;
}

pub fn solve_part2(rules : &std::collections::HashMap::<u32,Rule>, messages_lines : &Vec<String>) -> u64 {

  let mut retval = 0;

  for line in messages_lines {
    if rule_zero(&rules, &line) {
      
      retval += 1;
    }
  }
  retval
}

pub fn solve(filename : String) {
  let (rules,messages_lines) = load_rules(filename.to_string());
  println!("Day 19 part 1 is {}", solve_part1(&rules, &messages_lines));
  assert!(226==solve_part1(&rules, &messages_lines));
  println!("Day 19 part 2 is {}", solve_part2(&rules, &messages_lines));

}