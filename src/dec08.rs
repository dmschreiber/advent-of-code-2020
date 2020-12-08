use std::fs;
use std::collections::HashMap;

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let rules: Vec<String> = contents.lines().map(|s| (&*s).to_string() ).collect();

  rules
}

pub fn create_structure(rule_lines: &Vec<String>) -> Vec<Thing> {
  let mut things = <Vec<Thing>>::new();

  for r in rule_lines {
    let items : Vec<String> = r.split_ascii_whitespace().map(|s| (&*s).to_string() ).collect();
    let operator = items[0].to_string();
    let argument = &items[1];

    let new_thing = Thing {operator: operator, argument: argument.parse::<i32>().unwrap()};
    things.push(new_thing);
  }
  // println!("{:?}", things);
  return things;
}
#[derive(Debug)]
pub struct Thing {
  operator: String,
  argument: i32,
}

pub fn solve(things: &Vec<Thing>) -> i32 {
  return solve_part2_sub(things,None);
}

pub fn solve_part2_sub(things: &Vec<Thing>, change_index: Option<usize>) -> i32 {
  let mut retval = 0;
  let mut index = 0 as usize;
  let mut history = <HashMap<usize,bool>>::new();
  
  while history.get(&index) == None {
    history.insert(index, true);
    let v = things.get(index).unwrap();

    let op;
    if (change_index != None) && (Some(index) == change_index) && (v.operator == "nop") {
      op = "jmp".to_string();
    } else if (change_index != None) && (Some(index) == change_index) && (v.operator == "jmp") {
      op = "nop".to_string();
    } else {
      op = v.operator.clone();
    }

    if op == "nop" {
      index += 1;
    } else if op == "acc" {
       retval = v.argument + retval;
       index += 1;
    }  else if op == "jmp" {
      let tmp = (v.argument + index as i32) as usize;
      index = tmp;
    }  
      if index >= things.len() {
      println!("Success at {:?} Final accumulator {}, index {}, and {:?}", change_index, retval, index, history.get(&index));
      // retval = 0;
      break;
    }
  }
  if change_index == None {
    // running program as-is
    println!("Infinte loop at {:?} Final accumulator {}, index {}, and {:?}", change_index, retval, index, history.get(&index));
    return retval
  } else if change_index != None && index >= things.len() {
    // changed one operator and found success
    return retval
  } else
  {
    // changed one operator and hit infinte loop
    return 0
  }
}

pub fn solve_part2(things: &Vec<Thing>) -> i32{
  for (i,_t) in things.iter().enumerate() {
    let retval = solve_part2_sub(things,Some(i));

    if retval != 0 {
      return retval;
    }
  }
  return 0;
}