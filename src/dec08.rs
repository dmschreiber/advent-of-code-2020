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
  let mut retval = 0;
  let mut index = 0 as usize;
  let mut history = <HashMap<usize,bool>>::new();
  
  while history.get(&index) == None {
    history.insert(index, true);
    let v = things.get(index).unwrap();

    if v.operator == "nop" {
      index += 1;
    } else if v.operator == "acc" {
       retval += v.argument;
       index += 1;
    }  else if v.operator == "jmp" {
      let tmp = (v.argument + index as i32) as usize;
      // println!("trying to jump {} by {} get {}", index, v.argument, tmp);
      index = tmp;
    }  
    // println!("history {:?}", history);
  }



  println!("Final accumulator {}, index {}, and {:?}", retval, index, history.get(&index));
  retval
}

pub fn solve_part2_sub(things: &Vec<Thing>, change_index: usize) -> i32 {
  let mut retval = 0;
  let mut index = 0 as usize;
  let mut history = <HashMap<usize,bool>>::new();
  
  while history.get(&index) == None {
    history.insert(index, true);
    let v = things.get(index).unwrap();

    let mut op = v.operator.clone();

    if (index == change_index) && (op == "nop") {
      op = "jmp".to_string();
    }

    if (index == change_index) && (op == "jmp") {
      op = "nop".to_string();
    }

    if op == "nop" {
      index += 1;
    } else if op == "acc" {
       retval += v.argument;
       index += 1;
    }  else if op == "jmp" {
      let tmp = (v.argument + index as i32) as usize;
      index = tmp;
    }  
      if index >= things.len() {
      println!("Success at {} Final accumulator {}, index {}, and {:?}", change_index, retval, index, history.get(&index));
      retval = 0;
      break;
    }
  }
  // println!("Final accumulator {}, index {}, and {:?}", retval, index, history.get(&index));
  retval
}

pub fn solve_part2(things: &Vec<Thing>) {
  // solve_part2_sub(things,7);
  for (i,t) in things.iter().enumerate() {
    if solve_part2_sub(things,i) == 0 {
      break;
    }
  }
}