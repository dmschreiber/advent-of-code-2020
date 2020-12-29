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
  static ref RULE_OR_REGEX: Regex = Regex::new(r"^(\d+): (?P<lhs>(\d+ )+)\| (?P<rhs>(\d+\s*)+)$").unwrap();
  static ref RULE_SIMPLE_OR_REGEX: Regex = Regex::new(r"^(\d+): (\d+) \| (\d+)$").unwrap();

  static ref RULE_LITERAL_REGEX: Regex = Regex::new(r#"^(\d+): .([ab]).$"#).unwrap();
  
  static ref RULE_BASIC_REGEX: Regex = Regex::new(r"^(\d+): (?P<basic>(\d+\s*)+)$").unwrap();
  // static ref RULE_BASIC1_REGEX: Regex = Regex::new(r"^(\d+): (\d+)$").unwrap();
  
  // static ref RULE_BASIC2_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+)$").unwrap();
  // static ref RULE_BASIC3_REGEX: Regex = Regex::new(r"^(\d+): (\d+) (\d+) (\d+)$").unwrap();
}

#[derive(Debug,Clone,PartialEq)]
pub enum Rule {
  Or(Vec<u32>,Vec<u32>),
  Basic(Vec<u32>),
  Literal(String),  
}

fn do_basic(rules : &std::collections::HashMap<u32,Rule>, rule_list : Vec<u32>, line : Option<String>, level : Option<String>) -> Vec<Option<String>> {
  let mut retval = vec![line];

  for r_number in rule_list {
    if let Some(r) = rules.get(&r_number) {
      let mut new_list = vec![]; // generate a list of possible matches

      while let Some(result) = retval.pop() {
        new_list.append(&mut does_match(rules,r.clone(), result, level.clone()));
      }
      retval = new_list.iter().filter(|&a| *a != None).map(|a| a.clone()).collect::<Vec<Option<String>>>().clone();

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
    Rule::Literal(_l) if my_string.len() == 0 => { vec![None] }
    Rule::Literal(l) if *l == my_string[..1] => { vec![Some(format!("{}",&my_string[1..]).to_string())] }
    Rule::Literal(_l) => { vec![None] }
    Rule::Basic(v) => { do_basic(rules, v, Some(my_string),Some(format!("{}>",level_format))) }      
    Rule::Or(v1,v2) => 
    {
        let mut matches = do_basic(rules, v1, Some(my_string.clone()), Some(format!("{}>",level_format)));
        let mut or_matches = do_basic(rules, v2, Some(my_string.clone()),Some(format!("{}>",level_format))).clone();
        matches.append(&mut or_matches);
      
        matches
      }
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
  let mut rule_number;

  for line in rules_lines {
    // print!("RULES LINE {}", line);
    if let Some(args) = RULE_OR_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1 = args.name("lhs").unwrap().as_str().trim();
      let p2 = args.name("rhs").unwrap().as_str().trim();
      let v1 : Vec<u32> = p1.split(" ").map(|n| n.parse().unwrap() ).collect();
      let v2 : Vec<u32> = p2.split(" ").map(|n| n.parse().unwrap() ).collect();
      rules.insert(rule_number,Rule::Or(v1,v2));

    } else if let Some(args) = RULE_LITERAL_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let literal = args[2].to_string();
      rules.insert(rule_number, Rule::Literal(literal));
      // println!("LITERAL {} {:?}", rule_number, rules.get(&rule_number).unwrap());
    } else if let Some(args) = RULE_BASIC_REGEX.captures(line) {
      rule_number = args[1].parse::<u32>().unwrap();
      let p1 = args.name("basic").unwrap().as_str().trim();
      let v1 : Vec<u32> = p1.split(" ").map(|n| n.parse().unwrap() ).collect();
      rules.insert(rule_number, Rule::Basic(v1));

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

pub fn solve_part2(rules : &std::collections::HashMap::<u32,Rule>, messages_lines : &Vec<String>) -> u64 {

  let mut retval = 0;
  for line in messages_lines {
    if rule_zero(&rules, &line.to_string()) {
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