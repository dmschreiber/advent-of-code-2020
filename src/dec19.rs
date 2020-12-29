#[cfg(test)]
mod tests {
  
  #[test]
  pub fn dec19_prod() {
      let (mut rules,messages_lines) = super::load_rules("./inputs/dec19.txt".to_string());
      println!("Day 19 part 1 is {}", super::solve_part1(&rules, &messages_lines));
      assert!(226==super::solve_part1(&rules, &messages_lines));

      rules.insert(8,super::Rule::Or(vec![42],vec![42,8]));
      rules.insert(11,super::Rule::Or(vec![42,31],vec![42,11,31]));

      let val = super::solve_part2(&rules, &messages_lines);
      println!("part two prod 2 {}",val);
      assert!(val==355);

  }
  #[test]
  pub fn dec19_test() {
    let (rules,messages_lines) = super::load_rules("./inputs/dec19-test.txt".to_string());
    assert!(2==super::solve_part1(&rules, &messages_lines));

    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("ababbb".to_string()),None).contains(&Some("".to_string())));
    assert!(super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("abbbab".to_string()),None).contains( &Some("".to_string())));
    assert!(!super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("bababa".to_string()),None).contains(  &Some("".to_string())));
    assert!(!super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("aaabbb".to_string()),None).contains(&Some("".to_string())));
    assert!(!super::does_match(&rules,rules.get(&0).unwrap().clone(),Some("aaaabbb".to_string()),None).contains( &Some("".to_string())));

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
    
    rules.insert(8,super::Rule::Or(vec![42],vec![42,8]));
    rules.insert(11,super::Rule::Or(vec![42,31],vec![42,11,31]));

    for l in all_lines {
      let matched = super::rule_zero(&rules, &l.to_string()); 
      if matched && !matched_lines.contains(&l) {
        println!(" [[{} shouldn't match but does]]", &l);
      } else if !matched && matched_lines.contains(&l) {
        println!(" [[{} doesn't match but should]]", &l);
      }
    }
 
  
    let val = super::solve_part2(&rules, &messages_lines);
    println!("part two test should be 12 - {}",val);

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
pub enum Rule {
  Or(Vec<u32>,Vec<u32>),
  Basic(Vec<u32>),
  Literal(String),  
}

fn do_basic(rules : &std::collections::HashMap<u32,Rule>, rule_list : Vec<u32>, line : Option<String>, level : Option<String>) -> Vec<Option<String>> {
  let mut result; //  = line;
  let mut retval = vec![line];
  // let mut new_list = vec![];

  for r_number in rule_list {
    if let Some(r) = rules.get(&r_number) {
      result = retval.pop();
      let mut new_list = vec![];
      while result != None {
        new_list.append(&mut does_match(rules,r.clone(), result.unwrap(), level.clone())); // I need to handle the option that this could return multiple paths
        result = retval.pop();
      }
      retval = new_list.iter().filter(|&a| *a != None).map(|a| a.clone()).collect::<Vec<Option<String>>>().clone();
      // retval = new_list;

    }
  }
  return retval;
}

fn does_match (rules : &std::collections::HashMap<u32,Rule>, rule : Rule, line : Option<String>, level : Option<String>) -> Vec<Option<String>> {
  if line == None { return vec![None]; }
  let my_string = line.unwrap().clone();
  
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
    Rule::Literal(l) => 
          { if my_string.len() == 0 { vec![None] }
            else if *l == my_string[..1] { 
              vec![Some(format!("{}",&my_string[1..]).to_string())]
            } else {
              vec![None]
            }
          }
    Rule::Basic(v) => { do_basic(rules, v, Some(my_string),Some(format!("{}>",level_format))) }      
    Rule::Or(v1,v2) => 
    {
        let mut matches = do_basic(rules, v1, Some(my_string.clone()), Some(format!("{}>",level_format)));
        let mut or_matches = do_basic(rules, v2, Some(my_string.clone()),Some(format!("{}>",level_format))).clone();

        // println!("{} result of Or {:?} and {:?}",level_format,matches, or_matches);
        matches.append(&mut or_matches);
      
        matches
      }
    };
    // println!("returning {:?}", retval);
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
      rules.insert(rule_number,Rule::Or(vec![p1_a1,p1_a2],vec![p2_a1,p2_a2]));
    } else if let Some(args) = RULE_SIMPLE_OR_REGEX.captures(line) {
      // println!("SIMPLE OR");
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p2_a1 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(vec![p1_a1],vec![p2_a1]));
      // rules.insert(rule_number,Rule::SimpleOr(p1_a1,p2_a1));

    } else if let Some(args) = RULE_OR12_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p2_a1 = args[3].parse::<u32>().unwrap();
      let p2_a2 = args[4].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(vec![p1_a1],vec![p2_a1,p2_a2]));
    } else if let Some(args) = RULE_OR23_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1_a1 = args[2].parse::<u32>().unwrap();
      let p1_a2 = args[3].parse::<u32>().unwrap();
      let p2_a1 = args[4].parse::<u32>().unwrap();
      let p2_a2 = args[5].parse::<u32>().unwrap();
      let p2_a3 = args[6].parse::<u32>().unwrap();
      rules.insert(rule_number,Rule::Or(vec![p1_a1,p1_a2],vec![p2_a1,p2_a2,p2_a3]));

    } else if let Some(args) = RULE_LITERAL_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let literal = args[2].to_string();
      rules.insert(rule_number, Rule::Literal(literal));
      // println!("LITERAL {} {:?}", rule_number, rules.get(&rule_number).unwrap());
    } else if let Some(args) = RULE_BASIC3_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      let l3 = args[4].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Basic(vec![l1, l2, l3]));
      // println!("BASIC 3 {} {} {} {}", rule_number, &l1, &l2, &l3);
    } else if let Some(args) = RULE_BASIC2_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      let l2 = args[3].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Basic(vec![l1, l2]));
      // println!("BASIC 2 {} {} {}", rule_number, &l1, &l2);
    } else if let Some(args) = RULE_BASIC1_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let l1 = args[2].parse::<u32>().unwrap();
      rules.insert(rule_number, Rule::Basic(vec![l1]));
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
    // println!("{} - {:?}", line.len(), m);
    if m.contains(&Some("".to_string())) {
      retval += 1;
    }
  }
  retval
}


fn rule_zero(rules : &std::collections::HashMap::<u32,Rule>, messages_line : &String) -> bool {
  let m = does_match(&rules, rules.get(&0).unwrap().clone(),Some(messages_line.to_string()),None);
  return m.contains(&Some("".to_string()));
}

// let mut result;
//   // Rule 0 : 8 11
//   // Rule 8 : 42
//   // Rule 11 : 42 31

//   // new rules
//   // Rule 8 : 42 | 42 8
//   // Rule 11 : 42 31 | 42 11 31

//   result = does_match(&rules, rules.get(&0).unwrap().clone(),Some(messages_line.to_string()),None);
//   if result.contains(&Some("".to_string())) {
//     return true;
//   }

//   // Option 1 is just 42 42 31
//   result = does_match(&rules, Rule::Basic(vec![42,42,31]),Some(messages_line.to_string()),None);
//   if result.contains(&Some("".to_string())) {
//     return true;
//   }

//   // Option 2 is 42(n+2) 31
//   // 42 8 42 31 -> 42 (42 | 42 8) 42 31 -> 42 (42 | 42 (42 | 42 8)) 42 31
//   // 42 42 42 31, 42 42 42 42 31, ... 42 (n+2) times then 31 where n >= 1
//   // IN TEST Rule 42 matches 5 chars; Rule 31 matches 5 chars
//   // IN PROD Rule 42 and Rule 31 match 8 chars
//   // println!("LINE {}", messages_line);

//   for n in 1..=9 {
//     // Do Rule 42 n+2 times
//     let mut v = vec![42,42];
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     for _i in 0..n {
//       v.push(42);
//       // result = do_basic1(&rules, 42, result, None);
//     }
//     // Do Rule 31 one times
//     v.push(31);
//     // result = do_basic1(&rules, 31, result, None);

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }

//   // remaining option 3
//   // 42 42 11 31 -> 42 42 (42 31 | 42 11 31) 31 -> 42 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31 
//   //                                            -> 42 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31) 31
//   // 42 42 42 31 31, 42 42 42 42 31 31 31, 42 42 42 42 42 31 31 31 31, 
//   //                                       42x(n+2) and 31x(n+1) where n>=1 and n <= 5
//   // 42 42 42 42 42 42 31 31 31 31 31 is the longest that could match
//   // for all n>= 2 match 42 n+1 times and 31 n times (lengths 25, 35, 45, 55, etc chars IN TEST)
//   // Option 3 summary
//   // 42(n+2) 31(n+1) n>=1 n<=5

//   for n in 1..=6 {
//     // Do Rule 42 n+2 times
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     let mut v = vec![42,42];

//     for _i in 0..n {
//       // result = do_basic1(&rules, 42, result, None);
//       v.push(42);
//     }
//     // Do Rule 31 n+1 times
//     v.push(31);
//     // result = do_basic1(&rules, 31, result, None);
//     for _i in 0..n {
//       v.push(31);
//       // result = do_basic1(&rules, 31, result, None);
//     }

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }

//   // remaining option 4
//   // ******* NOTES ********
//   // 42 8 42 11 31 -> 42 (42 | 42 8) 42 (42 31 | 42 11 31) 31
//   //                  42 (42 | 42 (42 | 42 8)) 42 (42 31 | 42 (42 31 | 42 11 31) 31) 31

//   // option 4.1         opion 4.2                      option 4.3                            option 4.4
//   // 42 42 42 42 31 31, 42 42 8 42 42 31 31,           42 42 42 42 11 31 31,                
//   //                --> 42 42 (42 | 42 8) 42 42 31 31, 42 42 42 42 (42 31 | 42 11 31) 31 31, 
//   //                    42(n+3) 31 31 n>=1             42(n+4) 31(n+2) n>= 1                 
//   //  
//   // option 4.4
//   // 42 42 8 42 42 11 31 31
//   // 42 42 (42 | 42 8) 42 42 (42 31 | 42 11 31) 31 31

//   // option 4.4.1                option 4.4.2                  option 4.4.3                   
//   // 42 42 42 42 42 42 31 31 31, 42 42 42 8 42 42 42 31 31 31, 42 42 42 42 42 42 11 31 31 31, 
//   //                             42(n+6) 31 31 31 n>=1         42(n+6) 31(n+3) n>=1

//   // option 4.4.4
//   // 42 42 42 8 42 42 42 11 31 31 31
//   // 42 42 42 (42 | 42 8) 42 42 42 (42 31 | 42 11 31) 31 31 31
//   // 42x8 31x4, 42(n+8) 31x4, 42x8 31(n+4), 42x4 8 42x4 11 31x4 <-- too long
//   // ******* END NOTES ********
//   // Option 4 Summary (9 options)
//   // 
//   // 42x4 31x2
//   result = do_basic(&rules, vec![42, 42, 42, 42,31, 31],Some(messages_line.to_string()), None);
//   // result = do_basic3(&rules, 42, 42, 42, Some(messages_line.to_string()), None);
//   // result = do_basic1(&rules, 42, result, None);
//   // result = do_basic2(&rules, 31, 31, result, None);

//   if result.contains(&Some("".to_string())) {
//     return true;
//   }


//   // 42(n+3) 31x2 n>=1
//   for n in 1..=8 {
//     // Do Rule 42 n+3 times
//     let mut v = vec![42,42,42];

//     // result = do_basic3(&rules, 42, 42, 42, Some(messages_line.to_string()), None);
//     for _i in 0..n {
//     //   result = do_basic1(&rules, 42, result, None);
//       v.push(42);
//     }
//     v.push(31);
//     v.push(31);

//     // // Do Rule 31 two times
//     // result = do_basic2(&rules, 31, 31, result, None);

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }

//   // 42(n+4) 31(n+2) n>= 1
//   for n in 1..=5 {
//     // Do Rule 42 n+4 times
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     let mut v = vec![42,42,42,42];

//     // result = do_basic2(&rules, 42, 42, result, None);
//     for _i in 0..n {
//       v.push(42);
//       // result = do_basic1(&rules, 42, result, None);
//     }
//     // Do Rule 31 n+2 times
//     // result = do_basic2(&rules, 31, 31, result, None);
//     v.push(31);
//     v.push(31);

//     for _i in 0..n {
//       // result = do_basic1(&rules, 31, result, None);
//       v.push(31);
//     }

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }

//   // 42x6 31x3
//   // Do Rule 42 6 times
//   let v = vec![42,42,42,42,42,42,31,31,31];
//   // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//   // result = do_basic2(&rules, 42, 42, result, None);
//   // result = do_basic2(&rules, 42, 42, result, None);

//   // Do Rule 31 3 times
//   // result = do_basic2(&rules, 31, 31, result, None);
//   // result = do_basic1(&rules, 31, result, None);

//   result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//   if result.contains(&Some("".to_string())) {
//     return true;
//   }

//   // 42(n+6) 31x3 n>=1
//   for n in 1..=5 {
//     let mut v = vec![42,42,42,42,42,42];
//     // Do Rule 42 n+6 times
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     for _i in 0..n {
//       v.push(42);
//       // result = do_basic1(&rules, 42, result, None);
//     }
//     // Do Rule 31 3 times
//   // Do Rule 31 3 times
//     v.push(31);
//     v.push(31);
//     v.push(31);
//     // result = do_basic2(&rules, 31, 31, result, None);
//     // result = do_basic1(&rules, 31, result, None);

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }

//   // 42(n+6) 31(n+3) n>=1
//   for n in 1..=5 {
//     let mut v = vec![42,42,42,42,42,42];
//     // Do Rule 42 n+6 times
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     for _i in 0..n {
//       v.push(42);
//       // result = do_basic1(&rules, 42, result, None);
//     }

//     // Do Rule 31 n+3 times
//     v.push(31);
//     v.push(31);
//     v.push(31);

//     // result = do_basic3(&rules, 31, 31, 31, result, None);
//     for _i in 0..n {
//       v.push(31);
//       // result = do_basic1(&rules, 31, result, None);
//     }

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }  
//   // 42x8 31x4
//   // Do Rule 42 8 times
//   let v = vec![42,42,42,42,42,42,42,42,31,31,31,31];
//   // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//   // result = do_basic2(&rules, 42, 42, result, None);
//   // result = do_basic2(&rules, 42, 42, result, None);
//   // result = do_basic2(&rules, 42, 42, result, None);

//   // // Do Rule 31 4 times
//   // result = do_basic2(&rules, 31, 31, result, None);
//   // result = do_basic2(&rules, 31, 31, result, None);

//   result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//   if result.contains(&Some("".to_string())) {
//     return true;
//   }

//   // 42(n+8) 31x4 n>=1
//   for n in 1..=5 {
//     let mut v = vec![42,42,42,42,42,42,42,42];
//     // Do Rule 42 n+8 times
//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     for _i in 0..n {
//       v.push(42);
//       // result = do_basic1(&rules, 42, result, None);
//     }

//     // Do Rule 31 4 times
//     v.push(31);
//     v.push(31);
//     v.push(31);
//     v.push(31);
//     // result = do_basic2(&rules, 31, 31, result, None);
//     // result = do_basic2(&rules, 31, 31, result, None);

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }  

//   // 42x8 31(n+4) n>=1
//   for n in 1..=5 {
//     // Do Rule 42 n+8 times
//     let mut v = vec![42,42,42,42,42,42,42,42];

//     // result = do_basic2(&rules, 42, 42, Some(messages_line.to_string()), None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);
//     // result = do_basic2(&rules, 42, 42, result, None);

//     // Do Rule 31 n+4 times
//     v.push(31);
//     v.push(31);
//     v.push(31);
//     v.push(31);
//     // result = do_basic2(&rules, 31, 31, result, None);
//     // result = do_basic2(&rules, 31, 31, result, None);
//     for _i in 0..n {
//       v.push(31);
//       // result = do_basic1(&rules, 31, result, None);
//     }

//     result = do_basic(&rules, v, Some(messages_line.to_string()), None);
//     if result.contains(&Some("".to_string())) {
//       return true;
//     }
//   }  

//   return false;
// }

pub fn solve_part2(rules : &std::collections::HashMap::<u32,Rule>, messages_lines : &Vec<String>) -> u64 {

  let mut retval = 0;
  for line in messages_lines {
    let m = does_match(&rules, rules.get(&0).unwrap().clone(),Some(line.to_string()),None);
    if m.contains(&Some("".to_string())) {
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