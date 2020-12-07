use std::fs;
use std::collections::HashMap;


const GOLD : &str = "shiny gold";

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let rules: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();


  rules
}

pub fn create_structure(rule_lines: &Vec<String>) -> HashMap<String,BagRule> {
  let mut rules = <HashMap<String,BagRule>>::new();
  for r in rule_lines {

    let parts = r.split("contain").collect::<Vec<&str>>();

    let containing_bag = get_bag_name(parts[0].to_string());
    let contained_bags = parts[1].split(", ").map(|s| (&*s).to_string() ).collect();
    
    rules.insert (containing_bag.clone(), BagRule{ contained_bag: containing_bag.to_string(), bags_contained: contained_bags, 
        has_gold: parts[1].find(GOLD) != None,
        has_no_other_bags: parts[1].find("no other bags") != None });

  }

  return rules;
}


pub struct BagRule {
  contained_bag: String,
  bags_contained: Vec<String>,
  has_gold: bool,
  has_no_other_bags: bool,
}

fn is_string_numeric(str: String) -> bool {
  for c in str.chars() {
      if !c.is_numeric() {
          return false;
      }
  }
  return true;
}

fn get_bag_count(s: String) -> u32 {
  // println!("Get bag count {}", s);
  let pieces = s.split_ascii_whitespace().collect::<Vec<&str>>();
  if is_string_numeric(pieces[0].to_string()) {
    return pieces[0].parse::<u32>().unwrap();
  } else {
    return 0;
  }
}

fn get_bag_name(s: String) -> String {

  let pieces = s.split_ascii_whitespace().collect::<Vec<&str>>();
  if is_string_numeric(pieces[0].to_string()) {
    return format!("{} {}", pieces[1],pieces[2]);
  } else {
    return format!("{} {}", pieces[0],pieces[1]);

  }
}

fn does_contain(rules: &HashMap<String,BagRule>, which: String) -> u32 {

  let rule;
  if let Some(i) = rules.get(&which) {
    rule = i;
  } else {
    return 0;
  }
      if rule.has_gold {
        return 1;
      } else {

        for bag in &rule.bags_contained {
          let bag_name = get_bag_name(bag.to_string());

          if does_contain(rules, bag_name) > 0 {
            return 1;
          }
        }

      }
      return 0;
}

pub fn solve(rules: &HashMap<String,BagRule>) -> u32 {

  let bags = rules.values().map(|rule| does_contain(rules, rule.contained_bag.to_string())).sum::<u32>();

  println!("{} bags can contain shiny gold", bags);
  return bags;
}

fn how_many_bags(rules: &HashMap<String,BagRule>, which: &String, how_many: u32) -> u32 {
  
  let rule;
  if let Some(i) = rules.get(which) {
    rule = i;
  } else {
    return 0;
  }

  if rule.has_no_other_bags {
    // println!("{}{} has no other bags returning {}", level, parts[0].to_string(), how_many);
    return how_many;
  } else {

    let sub_bag_count = rule.bags_contained.iter().map(
      |bag| 
      how_many_bags(rules, &get_bag_name(bag.to_string()), get_bag_count(bag.to_string()))
        ).sum::<u32>();

    let ret_val = how_many * sub_bag_count + how_many;
    
    // println!("{}done {} returning {}", level, containing_bag, ret_val);
    if rule.contained_bag == GOLD {
      return ret_val - 1;
    } else {
      return ret_val;
    }
  }
}

pub fn solve_part2(rules: &HashMap<String,BagRule>) -> u32 {

  let answer = how_many_bags(&rules, &GOLD.to_string(), 1);
  println!("shiny gold has {} bags", answer);
  return answer;

}
