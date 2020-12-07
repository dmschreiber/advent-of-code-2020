use std::fs;

const GOLD : &str = "shiny gold";

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let rules: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();


  rules
}

pub fn create_structure(rule_lines: &Vec<String>) -> Vec<BagRule> {
  let mut rules = <Vec<BagRule>>::new();
  for r in rule_lines {

    let parts = r.split("contain").collect::<Vec<&str>>();

    let containing_bag = get_bag_name(parts[0].to_string());
    let contained_bags = parts[1].split(", ").map(|s| (&*s).to_string() ).collect();
    
    rules.push(BagRule{ contained_bag: containing_bag, bags_contained: contained_bags, 
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

fn does_contain(rules: &Vec<BagRule>, which: String) -> bool {
  for rule in rules {
    if rule.contained_bag == which {

      if rule.has_gold {
        return true;
      } else {

        for bag in &rule.bags_contained {
          let bag_name = get_bag_name(bag.to_string());

          if does_contain(rules, bag_name) {
            return true;
          }
        }

      }
      return false;
    }
  }
  return false;
}

pub fn solve(rules: &Vec<BagRule>) -> u32 {
  let mut bags = 0;

  for rule in rules {
    if rule.has_gold {
      bags += 1;
    } else {

      for bag in &rule.bags_contained {
        let bag_name = get_bag_name(bag.to_string());
        if does_contain(rules, bag_name) {
          bags +=1;
          // println!("Bag counts up to {}", bags);
          break;
        }
      }
    }
  }
  println!("{} bags can contain shiny gold", bags);
  return bags;
}

fn how_many_bags(rules: &Vec<BagRule>, which: &String, how_many: u32, level: String) -> u32 {
  for rule in rules {

    if rule.contained_bag == *which {
      // println!("{}working {}x{}", level,how_many,containing_bag);

      if rule.has_no_other_bags {
        // println!("{}{} has no other bags returning {}", level, parts[0].to_string(), how_many);
        return how_many;
      } else {
        let mut sub_bag_count = 0;

        for bag in &rule.bags_contained {
          let bag_name = get_bag_name(bag.to_string());
          // sub_bag_count += get_bag_count(bag.to_string());

          sub_bag_count += how_many_bags(rules, &bag_name, get_bag_count(bag.to_string()),format!("-{}",level));
        }
        let ret_val = how_many * sub_bag_count + how_many;
        // println!("{}done {} returning {}", level, containing_bag, ret_val);
        if rule.contained_bag == GOLD {
          return ret_val - 1;
        } else {
          return ret_val;
        }
      }
    }
  }
  return 0;
}

pub fn solve_part2(rules: &Vec<BagRule>) -> u32 {

  let answer = how_many_bags(rules, &GOLD.to_string(), 1, "".to_string());
  println!("shiny gold has {} bags", answer);
  return answer;

}
