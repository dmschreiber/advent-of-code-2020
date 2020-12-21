#[cfg(test)]
mod tests {

  #[test]
  pub fn dec21_prod() {
    assert!(2162==super::solve_part1("./inputs/dec21.txt".to_string()));
    // super::solve_part2("./inputs/dec21.txt".to_string());

  }
  #[test]
  pub fn dec21_test() {
      assert!(5==super::solve_part1("./inputs/dec21-test.txt".to_string()));
  }  
}

use std::fs;
use regex::Regex;
use std::time::{Instant};
use std::collections::HashMap;

// 
lazy_static! {
  static ref FOOD_REGEX: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();

}

#[derive(Debug,Clone,PartialEq)]
pub struct Food {
  ingredients : Vec<String>,
  allergens : Vec<String>,
}

fn find_common(v1 : &Vec<String>, v2 : &Vec<String>, ignore : &Vec<String>) -> Vec<String> {
  let mut retval = vec![];

  for s in v1 {
    if v2.contains(&s) {
      if !ignore.contains(&s) {
        retval.push(s.clone());
      }
    }
  }
  
  retval
}


pub fn reduce_list (things : &Vec<Food>, allergen_map : &mut HashMap::<String,String>, filter_by : Option<String>) -> Vec<Food> {
  let mut v = vec![];
  let mut should_continue = false;
  let mut unique_allergens = vec![];

  let mut allergen_count = HashMap::<String,u32>::new();
  for t in things {
    for a in &t.allergens {
      if let Some(unique_a) = allergen_count.get_mut(a) {
        *unique_a = *unique_a + 1;
      } else {
        allergen_count.insert(a.clone(),1);
      }
    }
  }

  // Can skip matching myself unless an allergen only appears once
  let fast_forward = allergen_count.values().filter(|a| **a == 1).collect::<Vec<_>>().len() == 0;

  let my_things = things.iter().filter(|a| filter_by == None || a.allergens[0] == filter_by.clone().unwrap() );
  if things.len() == 1 { return things.clone(); };
  
  for t1 in my_things {
    for t2 in things {
      if fast_forward {
        if t1 == t2 { break; }
      } else {
        if filter_by != None && t1 == t2 { break; } // works for prod and test; slower prod
      }

      let ignore_allergens = allergen_map.keys().map(|s| s.to_string()).collect();
      let ignore_ingredients = allergen_map.values().map(|s| s.to_string()).collect();

      let result1 = find_common(&t1.allergens, &t2.allergens, &ignore_allergens);
      let result2 = find_common(&t1.ingredients,&t2.ingredients, &ignore_ingredients);

      if result1.len() == 1 && result2.len() == 1 {
        // println!("{:?} - {:?}", result1, result2);
        allergen_map.insert(result1[0].to_string(),result2[0].to_string());
      } else if result1.len() == 1 {
        let this_allergen = result1[0].clone();
        // println!("adding {:?} - {:?}", result1, result2);
        let f = Food { ingredients : result2, allergens : result1 };
        if !v.contains(&f) {
          v.push(f);
        }

        if !unique_allergens.contains(&this_allergen) {
          unique_allergens.push(this_allergen);
        }

        should_continue = true;
      }
    }
  }
  // v.sort_by(|a, b| a.allergens[0].cmp(&b.allergens[0] ));
  v.dedup_by(|a, b| format!("{}:{}",a.ingredients.join("~"),a.allergens.join("~"))
                ==  format!("{}:{}",b.ingredients.join("~"),b.allergens.join("~")));
  // println!("--> From {} to {} foods", things.len(), v.len());
  for (index,t) in v.iter().enumerate() {
    // println!("{:?} {:?}", t.allergens, t.ingredients);
    if index == 10 { break; }
  }

  if should_continue {
    if unique_allergens.len() == 1 {
      return reduce_list(&v, allergen_map,Some(unique_allergens[0].clone()));
    } else {
      let mut retval = vec![];
      // println!("Allergens {}", unique_allergens.join(", "));
      while allergen_map.keys().len() < unique_allergens.len() {
      // for _i in 0..2 {
        for a in &unique_allergens {
          retval = reduce_list(&v, allergen_map,Some(a.to_string()));

        }
      }
      return retval;
    }
  } else {
    return v;
  }

}

pub fn solve_part1(filename : String) -> u64 {
  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines : Vec<String> = contents.lines().map(|s| (&*s).to_string()).collect();
  
  let mut things = vec![];

  for line in lines {
    if let Some(args) = FOOD_REGEX.captures(&line) {
      let ingredients : Vec<String> = args[1].split(" ").map(|s| (*&s).to_string()).collect();
      let allergens : Vec<String> = args[2].split(", ").map(|s| (*&s).to_string()).collect();
      things.push(Food { ingredients : ingredients, allergens : allergens });
    }
  }
  
  // for t in things {
  //   println!("{:?} - {:?}", t.ingredients, t.allergens);
  // }

  let mut allergen_map = HashMap::<String,String>::new();

  reduce_list(&things, &mut allergen_map,None);

  let ingredients : Vec<String> = allergen_map.values().map(|s| s.to_string()).collect();
  let mut retval = 0;

  for t1 in &things {
    for i in &t1.ingredients {
      if !ingredients.contains(&i) {
        // print!("{},", i);
        retval += 1;
      }
    }
  }
  println!();
  // println!("Map {:?}", allergen_map);
  let mut critical_allergens : Vec<String> = allergen_map.keys().map(|s| s.to_string()).collect();
  critical_allergens.sort();
  println!("Day 21 Crtical allergens {:?}", critical_allergens);
  println!("Day 21 part 2 - Crtitical ingredients {}", critical_allergens.iter().map(|a| allergen_map.get(a).unwrap().to_string() ).collect::<Vec<String>>().join(","));

  return retval;
}