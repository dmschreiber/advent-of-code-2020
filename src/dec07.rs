use std::fs;


pub fn read_input(filename: String) -> Vec<String> {
  let actual_rules :Vec<BagRule>;

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let rules: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();

  // for r in rules {

  //   let parts = r.split("contain").collect::<Vec<&str>>();

  //   let containing_bag = get_bag_name(parts[0].to_string());
  //   let contained_bags = parts[1].split(", ").map(|s| (&*s).to_string() ).collect();
  //   actual_rules.push(BagRule{ contained_bag: containing_bag, bags_contained: contained_bags, has_gold: parts[1].find("shiny gold") != None });

  // }

  rules
}


struct BagRule {
  contained_bag: String,
  bags_contained: Vec<String>,
  has_gold: bool,
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

fn does_contain(rules: &Vec<String>, which: String) -> bool {
  for rule in rules {
    let parts = rule.split("contain").collect::<Vec<&str>>();
    let containing_bag = get_bag_name(parts[0].to_string());
    if containing_bag == which {

      if parts[1].find("shiny gold") != None {
        println!("{} has gold", parts[0].to_string());
        return true
      } else {
        let contained_bags = parts[1].split(", ").collect::<Vec<&str>>();
        for bag in contained_bags {
          let bag_name = get_bag_name(bag.to_string());

          if does_contain(rules, bag_name) {
            return true;
          }
        }
      }
    }
  }
  return false;
}

pub fn solve(rules: &Vec<String>) -> u32 {
  let mut bags = 0;

  for rule in rules {
    let parts = rule.split("contain").collect::<Vec<&str>>();

    if parts[1].find("shiny gold") != None {
      println!("{} has gold", parts[0].to_string());
      bags += 1;
    } else {

      let contained_bags = parts[1].split(", ").collect::<Vec<&str>>();
      for bag in contained_bags {
        let bag_name = get_bag_name(bag.to_string());
        println!("checking if {} has gold", bag_name);
        if does_contain(rules, bag_name) {
          bags +=1;
          println!("Bag counts up to {}", bags);
          break;
        }
      }
    }
  }
  println!("done  {} bags", bags);
  return bags;
}

fn how_many_bags(rules: &Vec<String>, which: &String, how_many: u32, level: String) -> u32 {
  for rule in rules {
    let parts = rule.split("contain").collect::<Vec<&str>>();
    let containing_bag = get_bag_name(parts[0].to_string());
    if containing_bag == *which {
      println!("{}working {}x{}", level,how_many,containing_bag);

      if parts[1].find("no other bags") != None {
        println!("{}{} has no other bags returning {}", level, parts[0].to_string(), how_many);
        return how_many;
      } else {
        let contained_bags = parts[1].split(", ").collect::<Vec<&str>>();
        let mut sub_bag_count = 0;

        for bag in contained_bags {
          let bag_name = get_bag_name(bag.to_string());
          // sub_bag_count += get_bag_count(bag.to_string());

          sub_bag_count += how_many_bags(rules, &bag_name, get_bag_count(bag.to_string()),format!("-{}",level));
        }
        let ret_val = how_many * sub_bag_count + how_many;
        println!("{}done {} returning {}", level, containing_bag, ret_val);
        if containing_bag == "shiny gold" {
          return ret_val - 1;
        } else {
          return ret_val;
        }
      }
    }
  }
  return 0;
}

pub fn solve_part2(rules: &Vec<String>) -> u32 {

  return how_many_bags(rules, &"shiny gold".to_string(), 1, "".to_string());

}
